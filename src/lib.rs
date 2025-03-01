//! # Backup Utility
//!
//! This is a backup utility that allows you to create backups of directories with various features
//! such as compression, incremental backups, integrity verification, and more.
//!
//! ## Features
//! - **Compression**: Compress the backup into a `.tar.gz` archive.
//! - **Incremental Backup**: Only copy files that have changed since the last backup.
//! - **Integrity Verification**: Verify the integrity of the backup using checksums.
//! - **Dry Run**: Simulate the backup process without actually copying files.
//! - **Logging**: Log the backup process for debugging and auditing.
//!
//! ## Usage
//! See the `main.rs` file for an example of how to use this library.

pub mod args;
pub mod backup;
pub mod config;
pub mod features;
pub mod permissions;

pub mod compression;
pub mod dry_run;
pub mod incremental;
pub mod integrity;
pub mod logging;
pub mod wildcards;

use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub use args::Args;

/// Represents errors that can occur during the backup process.
#[derive(Error, Debug)]
pub enum BackupError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    #[error("Source directory '{0}' does not exist")]
    SourceNotFound(String),
    #[error("Insufficient permissions for '{0}'")]
    PermissionDenied(String),
    #[error("Thread pool build error: {0}")]
    ThreadPool(#[from] rayon::ThreadPoolBuildError),
    #[error("Invalid number of threads: {0}")]
    InvalidThreads(String),
}

/// Creates a backup directory with a timestamp in ISO 8601 format.
///
/// # Arguments
/// - `source_path`: The path to the source directory.
/// - `target_path`: The path to the target directory where the backup will be stored.
///
/// # Returns
/// - `Ok(PathBuf)`: The path to the newly created backup directory.
/// - `Err(BackupError)`: If an error occurs during directory creation.
pub fn create_backup_dir(source_path: &Path, target_path: &Path) -> Result<PathBuf, BackupError> {
    let source_dir_name = source_path.file_name().unwrap().to_string_lossy();
    let timestamp = Local::now().format("%Y-%m-%dT%H-%M-%S").to_string();

    // Create the backup directory with a timestamp
    let backup_dir_with_timestamp = target_path.join(format!("{}_{}", source_dir_name, timestamp));

    // Create the directory
    fs::create_dir_all(&backup_dir_with_timestamp)?;

    Ok(backup_dir_with_timestamp)
}

/// Runs the backup process.
///
/// # Arguments
/// - `args`: The parsed command-line arguments.
///
/// # Returns
/// - `Ok(())`: If the backup process completes successfully.
/// - `Err(BackupError)`: If an error occurs during the backup process.
pub fn run_backup(args: args::Args) -> Result<(), BackupError> {
    // Check permissions for the source directory
    let source_path = Path::new(&args.source_dir);
    permissions::check_source_permissions(source_path)?;

    // Check permissions for the target directory
    let target_path = Path::new(&args.target_dir);
    permissions::check_target_permissions(target_path)?;

    // Load the configuration
    let config = config::load_config("default")?;

    // Create the backup directory with a timestamp
    let backup_dir_with_timestamp = create_backup_dir(source_path, target_path)?;

    // Create a vector of backup features
    let mut features: Vec<Box<dyn features::BackupFeature>> = vec![
        Box::new(wildcards::WildcardsFeature::new(
            &config.skip_folders_and_files,
            &config.skip_file_extensions,
        )),
        Box::new(compression::CompressionFeature {
            enabled: args.compress,
        }),
        Box::new(incremental::IncrementalFeature {
            enabled: args.incremental,
        }),
        Box::new(logging::LoggingFeature::new()),
        Box::new(dry_run::DryRunFeature {
            enabled: args.dry_run,
        }),
        Box::new(integrity::IntegrityFeature {
            enabled: args.verify,
        }),
    ];

    // Initialize the features
    for feature in &mut features {
        feature.initialize(&[]);
    }

    // Perform the backup
    backup::copy_directory(
        source_path,
        &backup_dir_with_timestamp,
        &config.skip_folders_and_files,
        &config.skip_file_extensions,
        &features,
        args.threads,
    )?;

    // Print a success message
    println!(
        "Backup completed: {} -> {:?}",
        args.source_dir, backup_dir_with_timestamp
    );

    Ok(())
}
