use std::{collections::HashMap, error::Error};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PypiDownloads {
    pub last_day: i64,
    pub last_month: i64,
    pub last_week: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PypiInfo {
    pub author: String,
    pub author_email: String,
    pub bugtrack_url: Option<String>,
    pub classifiers: Vec<String>,
    pub description: String,
    pub description_content_type: String,
    pub docs_url: Option<String>,
    pub download_url: String,
    pub downloads: PypiDownloads,
    pub home_page: String,
    pub keywords: String,
    pub license: String,
    pub maintainer: String,
    pub maintainer_email: String,
    pub name: String,
    pub package_url: String,
    pub platform: String,
    pub project_url: String,
    pub project_urls: HashMap<String, String>,
    pub release_url: String,
    pub requires_dist: Vec<String>,
    pub requires_python: String,
    pub summary: String,
    pub version: String,
    pub yanked: bool,
    pub yanked_reason: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PypiRelease {
    pub comment_text: String,
    pub digests: HashMap<String, String>,
    pub downloads: i64,
    pub filename: String,
    pub has_sig: bool,
    pub md5_digest: String,
    pub packagetype: String,
    pub python_version: String,
    pub requires_python: Option<String>,
    pub size: u64,
    pub upload_time: String,
    pub upload_time_iso_8601: String,
    pub url: String,
    pub yanked: bool,
    pub yanked_reson: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PypiProject {
    pub info: PypiInfo,
    pub last_serial: u64,
    pub releases: HashMap<String, Vec<PypiRelease>>,
    pub urls: Vec<PypiRelease>,
    pub vulnerabilities: Vec<String>,
}

pub async fn get_project_info(project: &str) -> Result<PypiProject, Box<dyn Error>> {
    Ok(
        reqwest::get(format!("https://pypi.org/pypi/{}/json", project))
            .await?
            .json()
            .await?,
    )
}
