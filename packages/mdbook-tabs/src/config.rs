// use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {}

impl Config {
    // pub fn parse_from_toml(content: &str) -> Result<Self, toml::de::Error> {
    //     toml::from_str(content)
    // }
}
