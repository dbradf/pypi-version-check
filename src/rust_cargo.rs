use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct GithubRepo {
    pub owner: String,
    pub repo: String,
}

impl GithubRepo {
    pub fn from_github_url(url: &str) -> Option<Self> {
        let parts: Vec<&str> = url.split('/').rev().collect();
        if parts.len() >= 3 {
            Some(GithubRepo {
                repo: parts[0].to_string(),
                owner: parts[1].to_string(),
            })
        } else {
            None
        }
    }

    pub fn identifier(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CargoPackage {
    pub version: String,
    pub repository: Option<String>,
}

impl CargoPackage {
    pub fn get_github_repo(&self) -> Option<GithubRepo> {
        if let Some(repository) = &self.repository {
            GithubRepo::from_github_url(repository)
        } else {
            None
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CargoConfig {
    pub package: CargoPackage,
}

pub fn read_cargo_toml(location: &Path) -> Result<CargoConfig> {
    let contents = std::fs::read_to_string(location)?;
    Ok(toml::from_str(&contents)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_github_url() {
        let url = "https://github.com/dbradf/pypi-version-check";

        let github_repo = GithubRepo::from_github_url(url);

        assert!(github_repo.is_some());
        let github_repo = github_repo.unwrap();

        assert_eq!(github_repo.owner, "dbradf");
        assert_eq!(github_repo.repo, "pypi-version-check");
    }
}
