use clap::{Parser, Subcommand};
use git2::{Repository, ObjectType};
use semver::Version;
use std::error::Error;
use std::fs;

/// Command-line arguments structure
#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

// Define subcommands for different file types
#[derive(Subcommand)]
enum Commands {
    /// Use for JSON version file
    Json {
        /// Sets the JSON file
        #[arg(short = 'f', long)]
        file: String,

        /// Sets the key in the JSON file
        #[arg(short = 'k', long)]
        key: String,
    },
    /// Use for plain version file
    Plain {
        /// Sets the plain text file
        #[arg(short = 'f', long)]
        file: String,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments
    let cli = Cli::parse();
    let repo = Repository::open(".")?;

    match &cli.command {
        // Handling JSON version file subcommand
        Some(Commands::Json { file, key }) => {
            compare_versions::<JsonVersionFile>(&repo, file, key)?;
        }
        // Handling plain version file subcommand
        Some(Commands::Plain { file }) => {
            compare_versions::<TextVersionFile>(&repo, file, "")?;
        }
        // No subcommand provided
        None => {}
    }
    Ok(())
}

// Function to check if a version string adheres to semver format
fn is_semver_version(version_str: &str) -> Result<(), &'static str> {
    if !semver::Version::parse(version_str).is_ok() {
        return Err("Version does not adhere to semver 🙈");
    }
    Ok(())
}

// Trait for extracting version from different types of version files
trait VersionFile {
    fn get_version(content: &[u8], key: &str) -> Result<Version, Box<dyn Error>>;
}

// Implementation for JSON version file
struct JsonVersionFile;

impl VersionFile for JsonVersionFile {
    fn get_version(content: &[u8], key: &str) -> Result<Version, Box<dyn Error>> {
        let data: serde_json::Value = serde_json::from_slice(content)?;
        let version_str = data[key].as_str().ok_or("Version not found")?;
        is_semver_version(version_str)?;
        let version = Version::parse(version_str)?;
        Ok(version)
    }
}

// Implementation for plain text version file
struct TextVersionFile;

impl VersionFile for TextVersionFile {
    fn get_version(content: &[u8], _key: &str) -> Result<Version, Box<dyn Error>> {
        let content_str = std::str::from_utf8(content)?;
        is_semver_version(content_str.trim())?;
        let version = Version::parse(content_str.trim())?;
        Ok(version)
    }
}

// Function to read version from file
fn get_current_version_from_file<F: VersionFile>(file: &str, key: &str) -> Result<Version, Box<dyn Error>> {
    let content = fs::read(file)?;
    let version = F::get_version(&content, key)?;
    Ok(version)
}

// Function to read version from content
fn get_current_version_from_content<F: VersionFile>(content: &[u8], key: &str) -> Result<Version, Box<dyn Error>> {
    let version = F::get_version(&content, key)?;
    Ok(version)
}

// Function to determine file type based on extension
fn determine_file_type(file_path: &str) -> Option<String> {
    if let Some(extension) = file_path.split('.').last() {
        match extension {
            "json" => Some("json".to_string()),
            "yaml" => Some("yaml".to_string()),
            "toml" => Some("toml".to_string()),
            _ => None,
        }
    } else {
        None
    }
}

// Function to get version from previous commit
fn get_version_from_previous_commit<F: VersionFile>(repo: &Repository, file: &str, key: &str) -> Result<Version, Box<dyn Error>> {

    let head = repo.head()?.peel_to_commit()?;
    let previous_commit = head.parent(0)?; // Get the first parent (previous commit)
    let tree = previous_commit.tree()?;
    let file_name = tree.get_name(file)
        .ok_or("File not found in previous commit")?;
    let object = file_name.to_object(&repo)?;
    let blob = object.peel(ObjectType::Blob)?;
    let content = blob.as_blob().ok_or("Not a blob")?.content();

    let file_type = determine_file_type(file);

    let version = match file_type {
        Some(file_type) => {
            if file_type == "json" {
                get_current_version_from_content::<JsonVersionFile>(content, key)?
            } else {
                get_current_version_from_content::<TextVersionFile>(content, key)?
            }
        }
        _ => {
            return Err("Unknown file type".into());
        }
    };

    Ok(version)
}

// Function to compare current and previous versions
fn compare_versions<F: VersionFile>(repo: &Repository, file: &str, key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_type = determine_file_type(file);
    let current_version = match file_type {
        Some(file_type) => {
            if file_type == "json" {
                get_current_version_from_file::<JsonVersionFile>(file, key)?
            } else {
                get_current_version_from_file::<TextVersionFile>(file, key)?
            }
        }
        _ => {
            return Err("Unknown file type".into());
        }
    };

    let previous_commit_version = get_version_from_previous_commit::<F>(repo, file, key)?;
    if previous_commit_version >= current_version {
        return Err(format!("Current version ({}) is not greater than previous version ({}) 🦆", current_version, previous_commit_version).into());
    }
    println!("Current version is greater than the previous one 🚀🚀🚀");
    Ok(())
}
