//! This module provides the `IncrementalFeature` struct, which is used to implement
//! incremental backup functionality. It ensures that only files that have been modified
//! since the last backup are copied, reducing redundant operations.

use crate::features::BackupFeature;
use std::fs;
use std::path::Path;

/// The `IncrementalFeature` struct is used to enable or disable incremental backup functionality.
/// When enabled, it checks the modification timestamps of source and destination files
/// to determine whether a file needs to be copied.
///
/// # Fields
/// - `enabled`: A boolean flag that determines whether incremental backup is active.
pub struct IncrementalFeature {
    pub enabled: bool,
}

impl BackupFeature for IncrementalFeature {
    /// Processes a file during the backup operation, applying incremental backup logic
    /// if the feature is enabled. This method compares the modification timestamps of
    /// the source and destination files. If the source file has not been modified since
    /// the last backup, it is skipped.
    ///
    /// # Arguments
    /// - `src`: The source path of the file.
    /// - `dest`: The destination path where the file will be backed up.
    /// - `is_dir`: A boolean indicating whether the source is a directory.
    /// - `_features`: A slice of additional backup features (unused in this implementation).
    ///
    /// # Returns
    /// - `Ok(true)`: If the file should be processed by subsequent features (e.g., it has been modified).
    /// - `Ok(false)`: If the file should be skipped (e.g., it has not been modified).
    /// - `Err(crate::BackupError)`: If an error occurs while checking file metadata.
    fn process_file(
        &self,
        src: &Path,
        dest: &Path,
        is_dir: bool,
        _features: &[Box<dyn BackupFeature>],
    ) -> Result<bool, crate::BackupError> {
        // Skip directories and disabled incremental backups.
        if self.enabled && !is_dir {
            // Check if the destination file exists.
            if let Ok(dest_meta) = fs::metadata(dest) {
                // Get metadata for the source file.
                let src_meta = fs::metadata(src)?;

                // Compare modification timestamps.
                if src_meta.modified()? <= dest_meta.modified()? {
                    // Skip the file if it has not been modified since the last backup.
                    return Ok(false);
                }
            }
        }
        // Proceed with processing the file.
        Ok(true)
    }
}
