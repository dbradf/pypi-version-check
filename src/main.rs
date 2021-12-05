mod changelog;
mod pypi;
mod pypoetry;

use ansi_term::Color;
use pypi::get_project_info;
use pypoetry::read_pyproject_toml;
use simple_error::bail;
use std::{error::Error, path::PathBuf};
use structopt::StructOpt;

use crate::changelog::check_for_changelog;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pypi-version-check ",
    about = "Check if the version defined in pyproject.toml already exists in pypi"
)]
struct Opt {
    /// Path to directory containing pyproject.toml.
    #[structopt(long, default_value = ".")]
    project_path: PathBuf,

    /// Check for a changelog entry with the specified version
    #[structopt(long)]
    check_changelog: bool,

    // Name of changelog file to check.
    #[structopt(long, default_value = "CHANGELOG.md")]
    changelog_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let mut project_path = opt.project_path.clone();
    project_path.push("pyproject.toml");

    let pyproject = read_pyproject_toml(project_path.as_path())?;
    let project_name = &pyproject.tool.poetry.name;
    let version = &pyproject.tool.poetry.version;

    let info = get_project_info(project_name).await?;
    println!("Checking for version: {}", Color::Cyan.paint(version));
    if info.releases.contains_key(version) {
        println!(
            "{}",
            Color::Red.paint(format!(
                "Version {} already exists, latest version is {}",
                version, info.info.version
            ))
        );
        bail!("Conflicting version found")
    }

    if opt.check_changelog {
        println!(
            "Checking changelog at: {}",
            Color::Cyan.paint(&opt.changelog_name)
        );
        if !check_for_changelog(opt.project_path.as_path(), &opt.changelog_name, version)? {
            println!(
                "{}",
                Color::Red.paint(format!(
                    "Unable to find version {} in {}",
                    version, &opt.changelog_name
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
