use std::{path::Path, process::Command, str};

use anyhow::{bail, Result};
use cargo::core::Workspace;
use htmlentity::entity::{encode, CharacterSet, EncodeType, ICodedDataTrait};
use log::{error, info};

use crate::config::Config;

pub fn iframe(config: &Config) -> Result<String> {
    Ok(format!(
        "<iframe \
        data-mdbook-trunk=\"{}\" \
        class=\"mdbook-trunk-iframe\" \
        src=\"/{}/index.html{}{}\" \
        style=\"border: .1em solid var(--quote-border); border-radius: 5px; width: 100%;\"\
        {}></iframe>",
        encode(
            serde_json::to_string(config)?.as_bytes(),
            &EncodeType::Named,
            &CharacterSet::SpecialChars
        )
        .to_string()?,
        config.dest_name(),
        config
            .url_query
            .as_ref()
            .map(|query| format!("?{}", query.trim_start_matches("?")))
            .unwrap_or("".into()),
        config
            .url_fragment
            .as_ref()
            .map(|fragment| format!("#{}", fragment.trim_start_matches("#")))
            .unwrap_or("".into()),
        config
            .attributes
            .as_ref()
            .map(|attributes| attributes
                .iter()
                .filter_map(|(key, value)| encode(
                    value.as_bytes(),
                    &EncodeType::Named,
                    &CharacterSet::SpecialChars
                )
                .to_string()
                .ok()
                .map(|value| format!("{key}=\"{value}\"")))
                .collect::<Vec<_>>()
                .join(" "))
            .map(|s| format!(" {s}"))
            .unwrap_or("".into())
    ))
}

pub fn build(workspace: &Workspace, config: Config, dest_dir: &Path) -> Result<()> {
    let package_root = config.package_root(workspace)?;

    info!(
        "Building `{}` with feature(s) `{}` using Trunk.",
        config.package,
        config.features.join(", ")
    );

    let output = Command::new("trunk")
        .arg("build")
        .arg("--dist")
        .arg(dest_dir)
        .arg("--public-url")
        .arg(format!("/{}/", config.dest_name()))
        .arg("--no-default-features")
        .arg("--features")
        .arg(config.features.join(","))
        .current_dir(package_root)
        .output()?;

    if !output.status.success() {
        error!("{}", str::from_utf8(&output.stdout)?);
        bail!("Trunk build of package `{}` failed.", config.package);
    }

    Ok(())
}
