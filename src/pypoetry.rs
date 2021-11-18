use serde::Deserialize;
use std::{error::Error, path::Path};

#[derive(Deserialize, Debug, Clone)]
pub struct Poetry {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub readme: String,
    pub repository: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Tool {
    pub poetry: Poetry,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PyProjectConfig {
    pub tool: Tool,
}

pub fn read_pyproject_toml(location: &Path) -> Result<PyProjectConfig, Box<dyn Error>> {
    let contents = std::fs::read_to_string(location)?;
    Ok(toml::from_str(&contents)?)
}
