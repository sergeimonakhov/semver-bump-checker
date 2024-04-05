# Semver Bump Checker

Semver Bump Checker is a command-line tool written in Rust that helps compare versions stored in different file formats (JSON, plain text) with the versions from the previous commit in a Git repository.

## Features

- Supports comparison of versions stored in JSON and plain text files.
- Validates version strings to ensure they adhere to the Semantic Versioning (SemVer) specification.
- Compares the current version with the version from the previous commit in a Git repository.
- Provides informative error messages for various failure scenarios.

## Usage

```bash
Usage: sbc [COMMAND]

Commands:
  json   Use for JSON version file
  plain  Use for plain version file
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
