use serde::Deserialize;
use std::env;
use std::fs;

/// Struct representing the configuration loaded from the JSON file.
#[derive(Deserialize)]
pub struct Config {
    pub excluded_items: Vec<String>,
    pub excluded_extensions: Vec<String>,
}

/// Loads the configuration for a specific section from the JSON file located in the same directory as the executable.
///
/// # Parameters
/// - `section`: The section of the config (e.g., "python", "rust") to load.
///
/// # Returns
/// - A `Config` struct with excluded items and extensions.
/// - Returns `std::io::Result<Config>` which contains a Config instance if successful.
///
/// # Errors
/// - Will return an error if the config file cannot be read or parsed, or if the section does not exist.
///
/// # Panics
/// - Panics if the JSON file cannot be deserialized into the `Config` struct.
pub fn load_config(section: &str) -> std::io::Result<Config> {
    let exe_path = env::current_exe()?; // Get the path of the executable
    let config_path = exe_path.parent().unwrap().join("config.json"); // Construct the path to the configuration file

    let data = fs::read_to_string(config_path)?; // Read the config file as a string
    let all_config: serde_json::Value = serde_json::from_str(&data).expect("Error parsing JSON");

    // Try to get the specified section, fallback to default if section is not found
    let section_config = all_config
        .get(section)
        .or_else(|| all_config.get("default"))
        .expect("No section found in config, and no default section provided");

    // Deserialize the section into a Config struct
    let config: Config =
        serde_json::from_value(section_config.clone()).expect("Error parsing section config");

    Ok(config)
}
