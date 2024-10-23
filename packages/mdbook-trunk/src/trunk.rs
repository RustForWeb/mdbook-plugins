use std::{fs, path::Path, process::Command, str};

use anyhow::{bail, Result};
use cargo::core::Workspace;
use htmlentity::entity::{encode, CharacterSet, EncodeType, ICodedDataTrait};
use log::{error, info};

use crate::config::Config;

pub fn trunk(workspace: &Workspace, config: &Config) -> Result<String> {
    Ok(format!(
        "{}\n\n{}",
        iframe(config)?,
        files(workspace, config)?
    ))
}

pub fn iframe(config: &Config) -> Result<String> {
    Ok(format!(
        "<iframe \
        data-mdbook-trunk=\"{}\" \
        class=\"mdbook-trunk-iframe\" \
        src=\"/{}/index.html{}{}\" \
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
            .map(|query| format!("?{}", query.trim_start_matches('?')))
            .unwrap_or("".into()),
        config
            .url_fragment
            .as_ref()
            .map(|fragment| format!("#{}", fragment.trim_start_matches('#')))
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

fn files(workspace: &Workspace, config: &Config) -> Result<String> {
    let package_root = config.package_root(workspace)?;

    let mut header_elements: Vec<String> = vec![];
    let mut content_elements: Vec<String> = vec![];

    if let Some(files) = config.files.as_ref() {
        for (index, file) in files.iter().enumerate() {
            let file_path = package_root.join(file);

            info!(
                "Loading source file `{}`",
                file_path.to_str().unwrap_or_default()
            );

            let language = file_path.extension().and_then(|s| s.to_str()).unwrap_or("");
            let content = fs::read_to_string(&file_path)?;

            header_elements.push(format!(
                "<button class=\"mdbook-trunk-file{}\" data-file=\"{}\">{}</button>",
                (config.show_files.unwrap_or(false) && index == 0)
                    .then_some(" active")
                    .unwrap_or_default(),
                file,
                file_path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or(file),
            ));

            content_elements.push(format!(
                "<div class=\"mdbook-trunk-file-content{}\" data-file=\"{}\">\n\n```{}\n{}\n```\n\n</div>",
                (!(config.show_files.unwrap_or(false) && index == 0)).then_some(" hidden").unwrap_or_default(),
                file,
                language,
                content
            ));
        }
    }

    Ok(format!(
        "<div class=\"mdbook-trunk-files-container\">\n<nav class=\"mdbook-trunk-files\">\n<span class=\"mdbook-trunk-files-header\">Source code</span>\n{}\n</nav>\n{}\n</div>",
        header_elements.join("\n"),
        content_elements.join("\n")
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
