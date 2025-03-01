//! This module provides the `LoggingFeature` struct, which is used to log backup operations
//! to the console or other logging destinations. It uses the `log` and `simplelog` crates
//! to provide structured and configurable logging.

use crate::features::BackupFeature;
use log::{info, LevelFilter};
use simplelog::{Config, SimpleLogger};
use std::path::Path;

/// The `LoggingFeature` struct is used to enable logging during the backup process.
/// It logs file copy operations to the console with the `info` log level.
pub struct LoggingFeature;

impl LoggingFeature {
    /// Creates a new instance of `LoggingFeature` and initializes the logger.
    /// The logger is configured to log messages at the `info` level or higher.
    ///
    /// # Returns
    /// - A new instance of `LoggingFeature`.
    pub fn new() -> Self {
        // Initialize the logger with the `info` level and default configuration.
        SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();
        LoggingFeature
    }
}

impl Default for LoggingFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl BackupFeature for LoggingFeature {
    /// Logs file copy operations during the backup process. This method logs the source
    /// and destination paths of each file being copied.
    ///
    /// # Arguments
    /// - `src`: The source path of the file.
    /// - `dest`: The destination path where the file will be backed up.
    /// - `_is_dir`: A boolean indicating whether the source is a directory (unused in this implementation).
    /// - `_features`: A slice of additional backup features (unused in this implementation).
    ///
    /// # Returns
    /// - `Ok(true)`: Indicates that the file should be processed by subsequent features.
    fn process_file(
        &self,
        src: &Path,
        dest: &Path,
        _is_dir: bool,
        _features: &[Box<dyn BackupFeature>],
    ) -> Result<bool, crate::BackupError> {
        // Log the file copy operation.
        info!("Copying {} to {}", src.display(), dest.display());
        // Return `true` to indicate that the file should be processed by subsequent features.
        Ok(true)
    }
}
