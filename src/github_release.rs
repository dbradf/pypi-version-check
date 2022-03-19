use anyhow::Result;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GithubRelease {
    pub tag_name: String,
}

pub async fn get_release_info(owner: &str, repo: &str) -> Result<Vec<GithubRelease>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!(
            "https://api.github.com/repos/{}/{}/releases",
            owner, repo
        ))
        .header(USER_AGENT, "dbradf/pypi-version-check")
        .send()
        .await?;
    Ok(resp.json().await?)
}
