use std::{path::Path, process::Command, str};

use anyhow::{bail, Result};
use cargo::core::Workspace;
use htmlentity::entity::{encode, EncodeType, ICodedDataTrait};
use log::error;

use crate::config::Config;

pub fn iframe(config: &Config) -> Result<String> {
    Ok(format!(
        "<iframe data-mdbook-trunk=\"{}\" class=\"mdbook-trunk-iframe\" src=\"/{}/index.html\" style=\"border: none;\"></iframe>",
        encode(
            serde_json::to_string(config)?.as_bytes(),
            &EncodeType::Named,
            &htmlentity::entity::CharacterSet::SpecialChars
        )
        .to_string()?,
        config.dest_name(),
    ))
}

pub fn build(workspace: &Workspace, config: Config, dest_dir: &Path) -> Result<()> {
    let package_root = config.package_root(workspace)?;

    let output = Command::new("trunk")
        .arg("build")
        .arg("--dist")
        .arg(dest_dir)
        .arg("--public-url")
        .arg(format!("/{}/", config.dest_name()))
        .current_dir(package_root)
        .output()?;

    if !output.status.success() {
        error!("{}", str::from_utf8(&output.stdout)?);
        bail!("Trunk build of package `{}` failed.", config.package);
    }

    Ok(())
}
