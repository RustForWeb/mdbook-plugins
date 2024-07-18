use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TabsConfig {
    pub global: Option<String>,

    #[serde(skip)]
    pub tabs: Vec<(TabConfig, String)>,
}

impl TabsConfig {
    pub fn parse(content: &str) -> Result<Self, serde_keyvalue::ParseError> {
        serde_keyvalue::from_key_values(content)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TabConfig {
    pub name: String,
}

impl TabConfig {
    pub fn parse(content: &str) -> Result<Self, serde_keyvalue::ParseError> {
        serde_keyvalue::from_key_values(content)
    }
}
