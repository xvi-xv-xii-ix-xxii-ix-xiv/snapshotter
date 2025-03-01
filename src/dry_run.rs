//! This module provides the `DryRunFeature` struct, which is used to simulate file operations
//! without actually performing them. This is useful for testing and verifying backup operations
//! before executing them.

use crate::features::BackupFeature;
use std::path::Path;

/// The `DryRunFeature` struct is used to simulate file copy operations.
/// When enabled, it prints the source and destination paths to the console
/// without performing any actual file operations.
///
/// # Fields
/// - `enabled`: A boolean flag that determines whether the dry run mode is active.
pub struct DryRunFeature {
    pub enabled: bool,
}

impl BackupFeature for DryRunFeature {
    /// Simulates the file copy operation if the `enabled` flag is set to `true`.
    /// This method prints the source and destination paths to the console
    /// and returns `Ok(false)` to indicate that no actual operation was performed.
    /// If the `enabled` flag is `false`, it returns `Ok(true)` to indicate that
    /// the operation should proceed as normal.
    ///
    /// # Arguments
    /// - `src`: The source path of the file or directory.
    /// - `dest`: The destination path where the file or directory would be copied.
    /// - `_is_dir`: A boolean indicating whether the source is a directory (unused in this implementation).
    /// - `_features`: A slice of additional backup features (unused in this implementation).
    ///
    /// # Returns
    /// - `Ok(false)`: If the dry run mode is enabled, indicating no actual operation was performed.
    /// - `Ok(true)`: If the dry run mode is disabled, indicating the operation should proceed.
    /// - `Err(crate::BackupError)`: If an error occurs (not applicable in this implementation).
    fn process_file(
        &self,
        src: &Path,
        dest: &Path,
        _is_dir: bool,
        _features: &[Box<dyn BackupFeature>],
    ) -> Result<bool, crate::BackupError> {
        if self.enabled {
            // Print the simulated copy operation to the console.
            println!("Would copy: {} -> {}", src.display(), dest.display());
            // Return `false` to indicate no actual operation was performed.
            Ok(false)
        } else {
            // Return `true` to indicate the operation should proceed.
            Ok(true)
        }
    }
}
