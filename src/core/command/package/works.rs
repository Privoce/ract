use gen_utils::common::{exec_cmd, fs};
use gen_utils::error::Error;
use toml_edit::DocumentMut;

use std::path::Path;

use crate::core::entry::{FrameworkType, PackageFormat};
use crate::core::log::TerminalLogger;

#[cfg(target_os = "windows")]
pub fn specify_platform_with_works<P>(
    path: P,
    _dist_path: P,
    formats: Vec<PackageFormat>,
    _name: &str,
    _framework: Option<FrameworkType>,
) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // [check invalid package formats] ---------------------------------------------------------
    let invalid_format = formats.iter().any(|f| match f {
        PackageFormat::Default | PackageFormat::Nsis | PackageFormat::Wix => false,
        _ => true,
    });

    if invalid_format {
        return Err(Error::from("Invalid package formats in Windows"));
    }

    cargo_build(
        path.as_ref(),
        [],
        [("MAKEPAD_PACKAGE_DIR".to_string(), ".".to_string())],
    )?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn specify_platform_with_works<P>(
    path: P,
    dist_path: P,
    formats: Vec<PackageFormat>,
    name: &str,
    framework: Option<FrameworkType>,
) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    use gen_utils::common::exec_cmd;

    fn strip<P>(path: P, binary_path: &str) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let mut cmd = exec_cmd(
            "strip",
            [
                "--strip-unneeded",
                "--remove-section=.comment",
                "--remove-section=.note",
                binary_path,
            ],
            Some(path.as_ref()),
        );

        cmd.status().map_or_else(
            |e| {
                return Err(Error::from(e.to_string()));
            },
            |status| {
                if status.success() {
                    return Ok(());
                } else {
                    return Err(Error::from("strip failed!"));
                }
            },
        )
    }

    let prefix = if let Some(framework) = framework {
        match framework {
            FrameworkType::GenUI => ".",
            FrameworkType::Makepad => {
                if is_workspace(path.as_ref()) {
                    ".."
                } else {
                    "."
                }
            }
        }
    } else {
        "."
    };

    let binary_path = format!("{}/target/release/{}", prefix, &name);
    for format in formats {
        match format {
            PackageFormat::Default | PackageFormat::AppImage => {
                // [use default or appimage or pacman] ---------------------------------------------------
                cargo_build(
                    path.as_ref(),
                    [],
                    [("MAKEPAD_PACKAGE_DIR".to_string(), format!("lib/{}", name))],
                )?;
                let _ = strip(path.as_ref(), binary_path)?;
            }
            PackageFormat::Pacman => {
                cargo_build(
                    path.as_ref(),
                    [],
                    [(
                        "MAKEPAD_PACKAGE_DIR".to_string(),
                        format!("/usr/lib/{}", name),
                    )],
                )?;
                let _ = strip(path.as_ref(), binary_path)?;
            }
            PackageFormat::Deb => {
                // [in deb] ------------------------------------------------------------------------------
                cargo_build(
                    path.as_ref(),
                    [],
                    [(
                        "MAKEPAD_PACKAGE_DIR".to_string(),
                        format!("/usr/lib/{}", name),
                    )],
                )?;
                // use goblin to get the shared libraries
                deblib(binary_path, dist_path.as_ref())?;
                let _ = strip(path.as_ref(), binary_path)?;
            }
            _ => {
                return Err(Error::from("Invalid package formats in Linux"));
            }
        }
    }

    Ok(())
}

#[cfg(target_os = "macos")]
pub fn specify_platform_with_works<P>(
    path: P,
    _dist_path: P,
    formats: Vec<PackageFormat>,
    name: &str,
    framework: Option<FrameworkType>,
) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    use gen_utils::{
        common::{stream_cmd, stream_terminal},
        error::{ParseError, ParseType},
    };

    // [check invalid package formats] ---------------------------------------------------------
    let invalid_format = formats.iter().any(|f| match f {
        PackageFormat::Default | PackageFormat::App | PackageFormat::Dmg => false,
        _ => true,
    });

    if invalid_format {
        return Err(Error::Parse(ParseError::new(
            &format!(
                "Invalid package formats in Macos: {}",
                formats
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            ParseType::Conf,
        )));
    }
    // [cargo build] ----------------------------------------------------------------------------
    let extra_args = [];
    let mut extra_envs = vec![];

    let prefix = if let Some(framework) = framework {
        extra_envs.extend(vec![
            ("MAKEPAD".to_string(), "app_bundle".to_string()),
            ("MAKEPAD_PACKAGE_DIR".to_string(), ".".to_string()),
        ]);
        match framework {
            FrameworkType::GenUI => ".",
            FrameworkType::Makepad => {
                if is_workspace(path.as_ref()) {
                    ".."
                } else {
                    "."
                }
            }
        }
    } else {
        "."
    };
    cargo_build(path.as_ref(), extra_args, extra_envs)?;

    // [nstall_name_tool] --------------------------------------------------------------------------
    let binary_path = format!("{}/target/release/{}", prefix, &name);
    let mut cmd = stream_cmd(
        "install_name_tool",
        ["-add_rpath", "@executable_path/../Frameworks", &binary_path],
        Some(path.as_ref()),
    )
    .map_err(|e| e.to_string())?;

    return stream_terminal(
        &mut cmd,
        |line| TerminalLogger::new(&line).info(),
        |line| TerminalLogger::new(&line).warning(),
    )
    .map_or_else(
        |e| Err(e),
        |status| {
            if status.success() {
                Ok(())
            } else {
                Err(Error::from("install_name_tool failed!"))
            }
        },
    );
}

fn cargo_build<P, I, E>(path: P, extra_args: I, extra_envs: E) -> Result<(), Error>
where
    P: AsRef<Path>,
    I: IntoIterator<Item = String>,
    E: IntoIterator<Item = (String, String)>,
{
    let mut args = vec!["build".to_string(), "--release".to_string()];
    args.extend(extra_args);

    TerminalLogger::new("running cargo build, please wait ...").info();

    exec_cmd("cargo", args, Some(path))
        .envs(extra_envs)
        .status()
        .map_or_else(
            |e| Err(Error::from(e.to_string())),
            |status| {
                if status.success() {
                    TerminalLogger::new("cargo build successful").success();
                    Ok(())
                } else {
                    Err(Error::from("cargo build failed!"))
                }
            },
        )
}

#[cfg(target_os = "linux")]
fn deblib<P>(binary_path: &str, dist_path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    use goblin::elf::Elf;
    use std::fs::File;
    use std::io::{Read, Write};

    let mut file = File::open(binary_path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

    // parse ELF file
    let elf = Elf::parse(&buffer).map_err(|e| e.to_string())?;

    let mut dpkgs = Vec::new();

    for dyn_lib in elf.libraries {
        let lib_name = Path::new(dyn_lib)
            .file_name()
            .and_then(|f| f.to_str())
            .map(String::from);
        // skip if lib_name is None
        let Some(lib_name) = lib_name else {
            continue;
        };
        // use dpkg to find the package that provides the library
        let dpkg_output = exec_cmd("dpkg", ["-S", &lib_name], Option::<&Path>::None)
            .output()
            .map_err(|e| e.to_string())?;
        if !dpkg_output.status.success() {
            // skip if dpkg failed
            continue;
        }
        let dpkg_output = String::from_utf8_lossy(&dpkg_output.stdout);

        if let Some(package_name) = dpkg_output.split(":").next() {
            dpkgs.push(package_name.trim().to_string());
        }
    }

    // sort and de-duplicate dependencies
    dpkgs.sort();
    dpkgs.dedup();
    TerminalLogger::new(format!("Sorted and de-duplicated dependencies: {:#?}", dpkgs).as_str())
        .info();
    // 写入文件
    let deb_path = dist_path.as_ref().join("depends_deb.txt");
    let mut file = File::create(deb_path).map_err(|e| e.to_string())?;
    file.write_all(dpkgs.join("\n").as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(())
}

// judget current path is rust workspace or not
fn is_workspace<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    fn handle<P>(path: P, mut count: usize) -> Result<bool, Error>
    where
        P: AsRef<Path>,
    {
        count += 1;
        if count > 2 {
            return Ok(false);
        }
        let cargo_toml = path.as_ref().join("Cargo.toml");
        let toml = fs::read(cargo_toml)?
            .parse::<DocumentMut>()
            .map_err(|e| e.to_string())?;
        if toml.get("workspace").is_some() {
            Ok(true)
        } else {
            let pre_path = path
                .as_ref()
                .parent()
                .ok_or_else(|| Error::from("can not get parent path"))?;
            // handle
            handle(pre_path, count)
        }
    }

    handle(path, 0).unwrap_or(false)
}
