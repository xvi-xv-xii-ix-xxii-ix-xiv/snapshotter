use serde::Deserialize;
use std::fs;
use thiserror::Error;

/// Errors related to loading the configuration file.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// An I/O error occurred while reading the configuration file.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// An error occurred while parsing the YAML configuration file.
    #[error("Failed to parse YAML: {0}")]
    YamlParse(#[from] serde_yaml::Error),

    /// The specified section was not found in the configuration file, and no default section was provided.
    #[error("Section '{0}' not found and no default section provided")]
    SectionNotFound(String),
}

/// Represents the configuration loaded from the YAML file.
///
/// The configuration specifies which files and folders to skip during the backup process.
#[derive(Deserialize)]
pub struct Config {
    /// A list of folder and file names to skip during the backup.
    pub skip_folders_and_files: Vec<String>,

    /// A list of file extensions to skip during the backup.
    pub skip_file_extensions: Vec<String>,
}

/// Loads the configuration for a specific section from the YAML file.
///
/// The configuration file (`config.yaml`) should be located in the same directory as the executable.
/// If the specified section is not found, the function will attempt to load the `default` section.
///
/// # Parameters
/// - `section`: The section of the config to load (e.g., "python", "rust").
///
/// # Returns
/// - `Ok(Config)`: A `Config` struct containing the configuration for the specified section.
/// - `Err(ConfigError)`: If the configuration file cannot be read, parsed, or if the section does not exist.
///
/// # Examples
///
/// Example `config.yaml`:
/// ```yaml
/// default:
///   skip_folders_and_files:
///     - "temp"
///     - "*.log"
///   skip_file_extensions:
///     - "tmp"
///     - "bak"
///
/// python:
///   skip_folders_and_files:
///     - "__pycache__"
///   skip_file_extensions:
///     - "pyc"
/// ```
///
/// Loading the configuration:
/// ```rust
/// use snapshotter::config::load_config;
///
/// let config = load_config("python").unwrap();
/// println!("Skipping folders: {:?}", config.skip_folders_and_files);
/// ```
pub fn load_config(section: &str) -> Result<Config, ConfigError> {
    // Get the path to the executable
    let exe_path = std::env::current_exe()?;

    // Construct the path to the configuration file
    let config_path = exe_path.parent().unwrap().join("config.yaml");

    // Read the configuration file
    let data = fs::read_to_string(config_path)?;

    // Parse the YAML file into a serde_yaml::Value
    let all_config: serde_yaml::Value = serde_yaml::from_str(&data)?;

    // Get the specified section or fall back to the "default" section
    let section_config = all_config
        .get(section)
        .or_else(|| all_config.get("default"))
        .ok_or_else(|| ConfigError::SectionNotFound(section.to_string()))?;

    // Deserialize the section into a Config struct
    let config = serde_yaml::from_value(section_config.clone())?;

    Ok(config)
}
