use std::{collections::HashMap, path::PathBuf};

use anyhow::{Result, anyhow};
use cargo::{core::Workspace, ops::Packages};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileReplacement {
    pub find: String,
    pub replace: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub package: String,
    pub features: Vec<String>,
    pub files: Option<Vec<String>>,
    pub show_files: Option<bool>,
    pub file_replacements: Option<Vec<FileReplacement>>,
    pub url_query: Option<String>,
    pub url_fragment: Option<String>,
    pub attributes: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BuildConfig {
    pub package: String,
    pub features: Vec<String>,
}

impl BuildConfig {
    pub fn dest_name(&self) -> String {
        format!("{}--{}", self.package, self.features.join("--"))
    }

    pub fn package_root(&self, workspace: &Workspace) -> Result<PathBuf> {
        let packages = Packages::from_flags(false, vec![], vec![self.package.clone()])?
            .get_packages(workspace)?;

        if let Some(package) = packages.first() {
            Ok(package.root().to_path_buf())
        } else {
            Err(anyhow!(
                "Package `{}` not found in workspace.",
                self.package
            ))
        }
    }
}

impl Config {
    pub fn parse_from_json(content: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(content)
    }

    pub fn parse_from_toml(content: &str) -> Result<Self, toml::de::Error> {
        log::debug!("{content:?}");
        toml::from_str(content)
    }

    pub fn build_config(&self) -> BuildConfig {
        BuildConfig {
            package: self.package.clone(),
            features: self.features.clone(),
        }
    }
}
