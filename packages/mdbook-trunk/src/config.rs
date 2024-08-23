use std::{collections::HashMap, path::PathBuf};

use anyhow::{anyhow, Result};
use cargo::{core::Workspace, ops::Packages};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub package: String,
    pub features: Vec<String>,
    pub url_query: Option<String>,
    pub url_fragment: Option<String>,
    pub attributes: Option<HashMap<String, String>>,
}

impl Config {
    pub fn parse_from_json(content: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(content)
    }

    pub fn parse_from_toml(content: &str) -> Result<Self, toml::de::Error> {
        log::debug!("{:?}", content);
        toml::from_str(content)
    }

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
