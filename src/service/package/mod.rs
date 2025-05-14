mod info;
mod works;

use info::*;
use std::{
    collections::HashMap,
    env::current_dir,
    path::{Path, PathBuf},
    process::exit,
    str::FromStr,
};
use works::*;

use crate::entry::PackageFormat;
use crate::{
    entry::{FrameworkType, PackageConf, RactToml},
    log::{PackageLogs, TerminalLogger},
};
use cargo_metadata::MetadataCommand;
use gen_utils::{
    common::{
        exec_cmd,
        fs::{self, path_to_str},
        stream_cmd, stream_terminal,
    },
    error::Error,
};

use inquire::{Confirm, Select, Text};
use toml_edit::DocumentMut;
use which::which;

/// use cargo packager to package the makepad project
pub fn run() {
    PackageLogs::Welcome.terminal().info();
    PackageLogs::Desc.terminal().info();

    let _ = package().map_err(|e| {
        TerminalLogger::new(&e.to_string()).error();
        exit(2);
    });
}

fn package() -> Result<(), Error> {
    // [check cargo-packager is installed] -----------------------------------------------
    let _ = check_or_install_packager()?;
    // [init cargo-packager] -------------------------------------------------------------
    let _ = init_or_package()?;
    Ok(())
}

fn init_or_package() -> Result<(), Error> {
    PackageLogs::Init.terminal().info();
    // ask user need to init or not
    Select::new("Select how to package the project", vec!["init", "skip"])
        .prompt()
        .map_or_else(
            |e| Err(e.to_string().into()),
            |option| {
                match option {
                    "init" => {
                        // generate a Packager.toml
                        let info = generate_packager_toml()?;
                        // run cargo-packager
                        run_cargo_packager(info)
                    }
                    "skip" => {
                        let info = get_target_and_dist()?;
                        run_cargo_packager(info)
                    }
                    _ => Err("Invalid option".into()),
                }
            },
        )
}

pub fn check_or_install_packager() -> Result<(), Error> {
    // check if cargo-packager is installed
    if which("cargo-packager").is_ok() {
        Ok(())
    } else {
        // cargo install cargo-packager --locked
        PackageLogs::UnInstalled.terminal().warning();
        exec_cmd(
            "cargo",
            ["install", "cargo-packager", "--locked"],
            Option::<&Path>::None,
        )
        .status()
        .map_or_else(
            |e| Err(Error::from(e.to_string())),
            |status| {
                if status.success() {
                    PackageLogs::Installed.terminal().success();
                    Ok(())
                } else {
                    Err(
                        PackageLogs::InstallErr("cargo-packager install fail!".to_string())
                            .to_string()
                            .into(),
                    )
                }
            },
        )
    }
}

/// generate a Packager.toml
/// return (path_to_package, dist_path)
fn generate_packager_toml() -> Result<PackageInfo, Error> {
    // [get ract.toml] -----------------------------------------------------------------------------
    let ract_path = RactToml::path();

    let (path, framework, resources) = if ract_path.exists() {
        let ract: RactToml = (&ract_path).try_into()?;
        (
            match &ract.target {
                FrameworkType::GenUI => ract.first_compile()?.target.to_path_buf(),
                FrameworkType::Makepad => current_dir().unwrap(),
            },
            Some(ract.target),
            ract.resources,
        )
    } else {
        // maybe user use ract in other rust project
        (
            current_dir().unwrap(),
            None,
            FrameworkType::GenUI.resources_in_ract(),
        )
    };
    // [get package configuration] ----------------------------------------------------------------
    let mut conf = generate_package_conf(path.as_path(), framework.as_ref())?;
    // [write to Cargo.toml] -----------------------------------------------------------------------
    let generator = conf.generator(path.as_path(), framework);
    let _ = generator.generate(&conf)?;
    PackageLogs::PackageResourced.terminal().success();
    Ok(PackageInfo::new(path, conf, framework, resources))
}

fn generate_package_conf<P>(
    path: P,
    framework: Option<&FrameworkType>,
) -> Result<PackageConf, Error>
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
    let mut pack_conf = PackageConf::new(
        name,
        version,
        product_name,
        identifier,
        authors,
        license,
        framework,
    );
    pack_conf.publisher = publisher;
    pack_conf.description = desc.clone();
    pack_conf.long_description = desc;
    pack_conf.copyright = copyright;
    pack_conf.homepage = homepage;
    Ok(pack_conf)
}

fn run_cargo_packager(info: PackageInfo) -> Result<(), Error> {
    // ask user need to pack or stop
    let confirm = Confirm::new("Do you want to package the project now?")
        .with_help_message(
            "All of the configurations are generated, may be you need to do some modifications.",
        )
        .with_default(true)
        .prompt()
        .unwrap();

    if !confirm {
        return Ok(());
    }
    PackageLogs::Start.terminal().info();
    let resources = info.zip_resources();
    let PackageInfo {
        path,
        conf,
        framework,
        ..
    } = info;
    // [before package] ---------------------------------------------------------------------------
    let dist_resources_path = conf.dist_resources(framework.as_ref());
    fs::create_dir(&dist_resources_path)?;
    // [copy resources] ---------------------------------------------------------------------------
    let _ = copy_resources(resources, conf.path(framework.as_ref()))?;
    fs::copy(
        path.join("resources"),
        dist_resources_path.join(&conf.name).join("resources"),
    )?;
    TerminalLogger::new("copy all resources to dist resources successful").success();
    // [specify platform and do some works] -------------------------------------------------------
    let formats = conf
        .formats
        .as_ref()
        .cloned()
        .unwrap_or(vec![PackageFormat::Default]);
    specify_platform_with_works(
        path.as_path(),
        conf.out_dir.as_path(),
        formats,
        &conf.name,
        framework,
    )?;
    // [run cargo-packager] -----------------------------------------------------------------------
    let mut child =
        stream_cmd("cargo", ["packager", "--release"], Some(path)).map_err(|e| e.to_string())?;
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

fn copy_resources(
    resources: Option<HashMap<String, (PathBuf, PathBuf)>>,
    prefix: Option<PathBuf>,
) -> Result<(), Error> {
    if let Some(resources) = resources {
        let cargo_meta = MetadataCommand::new().exec().map_err(|e| e.to_string())?;
        let _ = cargo_meta.packages.iter().for_each(|package| {
            resources.get(&package.name).map(|(to_path, _)| {
                package.manifest_path.parent().map(|path| {
                    let to_path = if let Some(prefix) = prefix.as_ref() {
                        prefix.join(to_path)
                    } else {
                        to_path.to_path_buf()
                    }
                    .join("resources");
                    let from_path = path.join("resources");
                    TerminalLogger::new(&format!(
                        "copy resources from {} to {}",
                        path_to_str(from_path.as_path()),
                        path_to_str(to_path.as_path())
                    ))
                    .info();
                    // do copy, from_path -> to_path
                    let _ = fs::copy(from_path, to_path);
                })
            });
        });
    }
    Ok(())
}

fn get_target_and_dist() -> Result<PackageInfo, Error> {
    let ract_path = RactToml::path();
    // [get conf from target project Cargo.toml] ---------------------------------------------------
    let (target_path, framework, resources) = if !ract_path.exists() {
        (
            current_dir().unwrap(),
            None,
            FrameworkType::GenUI.resources_in_ract(),
        )
    } else {
        let ract = RactToml::try_from(&ract_path)?;
        (
            ract.first_compile()?.target.to_path_buf(),
            Some(ract.target),
            ract.resources,
        )
    };

    let conf = PackageConf::from_cargo_toml(&target_path.join("Cargo.toml"))?;

    Ok(PackageInfo::new(target_path, conf, framework, resources))
}
