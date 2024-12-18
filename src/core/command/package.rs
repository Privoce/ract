use std::{
    env::current_dir,
    path::{Path, PathBuf},
    process::{exit, Command, Stdio},
    str::FromStr,
};

use gen_utils::{
    common::{fs, stream_terminal},
    error::Error,
};
use inquire::{Confirm, Select, Text};
use toml_edit::DocumentMut;
use which::which;

use crate::core::{
    entry::PackageConf,
    log::{PackageLogs, TerminalLogger},
};

/// use cargo packager to package the makepad project
pub fn run() {
    PackageLogs::Welcome.terminal().rust();
    PackageLogs::Desc.terminal().info();

    let _ = run_packager().map_err(|e| {
        TerminalLogger::new(&e.to_string()).error();
        exit(2);
    });
}

fn run_packager() -> Result<(), Error> {
    // [check cargo-packager is installed] -----------------------------------------------
    let _ = check_and_install_packager()?;
    // [init cargo-packager] -------------------------------------------------------------
    let _ = init_packager()?;
    Ok(())
}

fn check_and_install_packager() -> Result<(), Error> {
    // check if cargo-packager is installed
    if which("cargo-packager").is_ok() {
        Ok(())
    } else {
        // cargo install cargo-packager --locked
        PackageLogs::UnInstalled.terminal().warning();
        Command::new("cargo")
            .args(&["install", "cargo-packager", "--locked"])
            .output()
            .map_or_else(
                |e| Err(Error::from(e.to_string())),
                |out| {
                    if out.status.success() {
                        PackageLogs::Installed.terminal().success();
                        Ok(())
                    } else {
                        Err(PackageLogs::InstallErr(
                            String::from_utf8_lossy(&out.stderr).to_string(),
                        )
                        .to_string()
                        .into())
                    }
                },
            )
    }
}

fn init_packager() -> Result<(), Error> {
    PackageLogs::Init.terminal().info();
    // ask user need to init or not
    Select::new("Select how to package the project", vec!["init", "skip"])
        .prompt()
        .map_or_else(
            |e| Err(e.to_string().into()),
            |option| {
                let path = current_dir().unwrap();
                // generate a Packager.toml
                match option {
                    "init" => generate_packager_toml(path.as_path()),
                    "skip" => run_cargo_packager(path.as_path()),
                    _ => Err("Invalid option".into()),
                }
            },
        )
}

fn generate_packager_toml<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    // [get name and version from Cargo.toml] ----------------------------------------------------
    let (name, version, authors, desc) = fs::read(path.as_ref().join("Cargo.toml")).map_or_else(
        |e| Err(Error::from(e.to_string())),
        |content| {
            let cargo_toml = content.parse::<DocumentMut>().map_err(|e| e.to_string())?;
            let name = cargo_toml["package"]["name"].as_str().unwrap().to_string();
            let version = cargo_toml["package"]["version"]
                .as_str()
                .unwrap()
                .to_string();
            let authors = cargo_toml["package"].get("authors").and_then(|authors| {
                authors.as_array().map(|authors| {
                    authors
                        .iter()
                        .map(|author| author.as_str().unwrap().to_string())
                        .collect::<Vec<String>>()
                })
            });

            let desc = cargo_toml["package"]["description"]
                .as_str()
                .map(|s| s.to_string());
            Ok((name, version, authors, desc))
        },
    )?;
    // [product-name] --------------------------------------------------------------------------------
    let product_name = Text::new("Input the product name")
        .with_default(&name)
        .prompt()
        .unwrap();
    // [identifier] -----------------------------------------------------------------------------------
    let identifier = Text::new("Input the identifier")
        .with_default(&format!("com.{}", &name))
        .prompt()
        .unwrap();
    // [license] --------------------------------------------------------------------------------------
    let license = Text::new("Path to the license file")
        .with_default("./LICENSE")
        .prompt_skippable()
        .unwrap()
        .map(|path| PathBuf::from_str(&path).unwrap());
    // [publisher] -------------------------------------------------------------------------------------
    let publisher = Text::new("Input the publisher name")
        .with_placeholder("you can enter to skip")
        .prompt_skippable()
        .unwrap();
    // [copyright] -------------------------------------------------------------------------------------
    let copyright = Text::new("Input the copy right")
        .with_placeholder("fmt: (Copyright YEAR, AUTHOR) you can enter to skip")
        .prompt_skippable()
        .unwrap();
    // [homepage] ---------------------------------------------------------------------------------------
    let homepage = Text::new("Input the homepage")
        .with_placeholder("you can enter to skip")
        .prompt_skippable()
        .unwrap();
    PackageLogs::Configing.terminal().info();
    let mut pack_conf = PackageConf::new(name, version, product_name, identifier, authors, license);
    pack_conf.publisher = publisher;
    pack_conf.description = desc.clone();
    pack_conf.long_description = desc;
    pack_conf.copyright = copyright;
    pack_conf.homepage = homepage;
    let generator = pack_conf.makepad(path.as_ref());
    // generate the packaging project and Packager.toml for makepad
    let _ = generator.makepad(pack_conf)?;
    PackageLogs::PackageResourced.terminal().success();
    // ask user need to pack or stop
    let confirm = Confirm::new("Do you want to package the project now?")
        .with_help_message(
            "All of the configurations are generated, may be you need to do some modifications.",
        )
        .with_default(true)
        .prompt()
        .unwrap();

    if confirm {
        // run cargo packager
        run_cargo_packager(path.as_ref())
    } else {
        Ok(())
    }
}

fn run_cargo_packager<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    PackageLogs::Start.terminal().info();
    // now directly run cargo-packager
    let mut child = Command::new("cargo")
        .args(&["packager", "--release"])
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    stream_terminal(
        &mut child,
        |line| TerminalLogger::new(&line).info(),
        |line| TerminalLogger::new(&line).warning(),
    )
    .map_or_else(
        |e| Err(e),
        |status| {
            if status.success() {
                PackageLogs::Confirm.terminal().success();
                Ok(())
            } else {
                Err(PackageLogs::Error.to_string().into())
            }
        },
    )
}
