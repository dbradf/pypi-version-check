# pypi-version-check

A tool to check if the poetry version of a python project (defined in pyproject.toml)
has already been uploaded to pypi. This is can be used as a test to ensure you don't
forget to update the version before trying to deploy it.

## Usage

```
pypi-version-check  0.2.0
Check if the version defined in pyproject.toml already exists in pypi

USAGE:
    pypi-version-check [FLAGS] [OPTIONS]

FLAGS:
        --check-changelog    Check for a changelog entry with the specified version
    -h, --help               Prints help information
    -V, --version            Prints version information

OPTIONS:
        --changelog-name <changelog-name>     [default: CHANGELOG.md]
        --project-path <project-path>        Path to directory containing pyproject.toml [default: .]      
```

When the version has been updated correctly:
```bash
$ pypi-version-check
Checking for version: 0.1.3
$ echo $?
0
```

When the version needs to be updated:
```bash
$ pypi-version-check
Checking for version: 0.1.2
Version 0.1.2 already exists, latest version is 0.1.2
Error: "Conflicting version found"
$ echo $?
1
```

Checking that the changelog was updated
```bash
$ pypi-version-check --check-changelog
Checking for version: 3.4.3
Checking changelog at: CHANGELOG.md
Unable to find version 3.4.3 in CHANGELOG.md
Changelog should contain version with the follow format:
        ## <version> - YYYY-MM-DD
Error: "Changelog file not updated"
```