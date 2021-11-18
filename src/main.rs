mod pypi;
mod pypoetry;

use pypi::get_project_info;
use pypoetry::read_pyproject_toml;
use simple_error::bail;
use std::{error::Error, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pypi-version-check ",
    about = "Check if the version defined in pyproject.toml already exists in pypi"
)]
struct Opt {
    /// Path to directory containing pyproject.toml.
    #[structopt(long, default_value = ".")]
    project_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let mut project_path = opt.project_path;
    project_path.push("pyproject.toml");

    let pyproject = read_pyproject_toml(project_path.as_path())?;
    let project_name = &pyproject.tool.poetry.name;
    let version = &pyproject.tool.poetry.version;

    let info = get_project_info(project_name).await?;
    println!("Checking for version: {}", version);
    if info.releases.contains_key(version) {
        println!(
            "Version {} already exists, latest version is {}",
            version, info.info.version
        );
        bail!("Conflicting version found")
    } else {
        Ok(())
    }
}
