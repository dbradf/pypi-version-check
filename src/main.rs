mod changelog;
mod github_release;
mod pypi;
mod pypoetry;
mod rust_cargo;

use ansi_term::Color;
use anyhow::Result;
use github_release::get_release_info;
use pypi::get_project_info;
use pypoetry::read_pyproject_toml;
use rust_cargo::read_cargo_toml;
use simple_error::bail;
use std::{
    error::Error,
    path::{Path, PathBuf},
};
use structopt::StructOpt;
use strum_macros::EnumString;

use crate::changelog::check_for_changelog;

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "kebab-case")]
enum PublishType {
    Pypi,
    GithubRelease,
}

impl PublishType {
    pub async fn get_versions(&self, project_name: &str) -> Result<Vec<String>> {
        match self {
            PublishType::Pypi => {
                let info = get_project_info(project_name).await?;
                Ok(info.releases.into_keys().collect())
            }
            PublishType::GithubRelease => {
                let (owner, repo) = project_name
                    .split_once('/')
                    .expect("Expected 'owner/repo' format for github project.");
                let info = get_release_info(owner, repo).await?;
                Ok(info
                    .into_iter()
                    .map(|r| r.tag_name.strip_prefix('v').unwrap().to_string())
                    .collect())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub published_name: String,
    pub current_version: String,
}

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "kebab-case")]
enum PackageType {
    PythonPoetry,
    Rust,
}

impl PackageType {
    pub fn get_current_version(&self, project_path: &Path) -> Result<VersionInfo> {
        Ok(match self {
            PackageType::PythonPoetry => {
                let mut project_path = project_path.to_path_buf();
                project_path.push("pyproject.toml");

                let pyproject = read_pyproject_toml(&project_path)?;
                let project_name = &pyproject.tool.poetry.name;
                let version = &pyproject.tool.poetry.version;
                VersionInfo {
                    published_name: project_name.to_string(),
                    current_version: version.to_string(),
                }
            }
            PackageType::Rust => {
                let mut project_path = project_path.to_path_buf();
                project_path.push("Cargo.toml");

                let cargo = read_cargo_toml(&project_path)?;
                let project_name = cargo
                    .package
                    .get_github_repo()
                    .expect("Github repo could not be found in 'Cargo.toml'")
                    .identifier();
                let version = cargo.package.version;

                VersionInfo {
                    published_name: project_name,
                    current_version: version,
                }
            }
        })
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pypi-version-check ",
    about = "Check if the current version of a project has already been published"
)]
struct Opt {
    /// Path to directory containing pyproject.toml.
    #[structopt(long, default_value = ".")]
    project_path: PathBuf,

    /// Check for a changelog entry with the specified version
    #[structopt(long)]
    check_changelog: bool,

    /// Name of changelog file to check.
    #[structopt(long, default_value = "CHANGELOG.md")]
    changelog_name: String,

    /// Type of publishing [pypi, github-release].
    #[structopt(long, default_value = "pypi")]
    publish_type: PublishType,

    /// Type of publishing [python-poetry, rust].
    #[structopt(long, default_value = "python-poetry")]
    package_type: PackageType,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let mut project_path = opt.project_path.clone();
    project_path.push("pyproject.toml");

    let version_info = opt.package_type.get_current_version(&opt.project_path)?;
    let published_versions = opt
        .publish_type
        .get_versions(&version_info.published_name)
        .await?;

    println!(
        "Checking for version: {}",
        Color::Cyan.paint(&version_info.current_version)
    );
    if published_versions.contains(&version_info.current_version) {
        println!(
            "{}",
            Color::Red.paint(format!(
                "Version {} already exists, latest version is {}",
                &version_info.current_version,
                published_versions.first().unwrap()
            ))
        );
        bail!("Conflicting version found")
    }

    if opt.check_changelog {
        println!(
            "Checking changelog at: {}",
            Color::Cyan.paint(&opt.changelog_name)
        );
        if !check_for_changelog(
            opt.project_path.as_path(),
            &opt.changelog_name,
            &version_info.current_version,
        )? {
            println!(
                "{}",
                Color::Red.paint(format!(
                    "Unable to find version {} in {}",
                    &version_info.current_version, &opt.changelog_name
                ))
            );
            println!("Changelog should contain version with the follow format: ");
            println!("\t## <version> - YYYY-MM-DD");
            bail!("Changelog file not updated")
        }
    }

    println!("{}", Color::Green.paint("Checks completed successfully!"));
    Ok(())
}
