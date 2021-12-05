use std::{error::Error, path::Path};

use regex::Regex;

pub fn check_for_changelog(
    project_path: &Path,
    change_log_name: &str,
    version: &str,
) -> Result<bool, Box<dyn Error>> {
    let mut path_buf = project_path.to_path_buf();
    path_buf.push(change_log_name);

    let version_regex = Regex::new(&format!("## {} - {}", version, r"\d{4}-\d{2}-\d{2}"))?;
    let contents = std::fs::read_to_string(path_buf)?;
    Ok(contents.lines().any(|l| version_regex.is_match(l)))
}
