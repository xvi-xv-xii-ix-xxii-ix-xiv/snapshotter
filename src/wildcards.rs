//! This module provides the `WildcardsFeature` struct, which is used to filter files and directories
//! during the backup process based on wildcard patterns. It allows skipping specific files, folders,
//! or file extensions that match the provided patterns.

use crate::features::BackupFeature;
use glob::Pattern;
use std::path::Path;

/// The `WildcardsFeature` struct is used to skip files, folders, or file extensions during the backup process
/// based on wildcard patterns. It supports glob-style patterns for matching.
///
/// # Fields
/// - `skip_folders_and_files`: A list of wildcard patterns for skipping specific files or folders.
/// - `skip_file_extensions`: A list of wildcard patterns for skipping specific file extensions.
pub struct WildcardsFeature {
    skip_folders_and_files: Vec<String>,
    skip_file_extensions: Vec<String>,
}

impl WildcardsFeature {
    /// Creates a new instance of `WildcardsFeature` with the provided lists of patterns.
    ///
    /// # Arguments
    /// - `skip_folders_and_files`: A list of wildcard patterns for skipping files or folders.
    /// - `skip_file_extensions`: A list of wildcard patterns for skipping file extensions.
    ///
    /// # Returns
    /// - A new instance of `WildcardsFeature`.
    pub fn new(skip_folders_and_files: &[String], skip_file_extensions: &[String]) -> Self {
        WildcardsFeature {
            skip_folders_and_files: skip_folders_and_files.to_vec(),
            skip_file_extensions: skip_file_extensions.to_vec(),
        }
    }
}

impl BackupFeature for WildcardsFeature {
    /// Processes a file or directory during the backup operation, skipping it if it matches
    /// any of the provided wildcard patterns for files, folders, or extensions.
    ///
    /// # Arguments
    /// - `src`: The source path of the file or directory.
    /// - `_dest`: The destination path (unused in this implementation).
    /// - `_is_dir`: A boolean indicating whether the source is a directory (unused in this implementation).
    /// - `_features`: A slice of additional backup features (unused in this implementation).
    ///
    /// # Returns
    /// - `Ok(true)`: If the file or directory should be processed by subsequent features.
    /// - `Ok(false)`: If the file or directory should be skipped.
    /// - `Err(crate::BackupError)`: If an error occurs (not applicable in this implementation).
    fn process_file(
        &self,
        src: &Path,
        _dest: &Path,
        _is_dir: bool,
        _features: &[Box<dyn BackupFeature>],
    ) -> Result<bool, crate::BackupError> {
        // Get the file name from the source path.
        let file_name = src.file_name().unwrap().to_string_lossy();

        // Check if the file or folder matches any of the skip patterns.
        if self
            .skip_folders_and_files
            .iter()
            .any(|pattern| Pattern::new(pattern).is_ok_and(|p| p.matches(&file_name)))
        {
            // Skip the file or folder if it matches a pattern.
            return Ok(false);
        }

        // Check if the file extension matches any of the skip patterns.
        if let Some(ext) = src.extension() {
            let ext_str = ext.to_string_lossy();
            if self
                .skip_file_extensions
                .iter()
                .any(|pattern| Pattern::new(pattern).is_ok_and(|p| p.matches(&ext_str)))
            {
                // Skip the file if its extension matches a pattern.
                return Ok(false);
            }
        }

        // If no patterns match, proceed with processing the file or folder.
        Ok(true)
    }
}
