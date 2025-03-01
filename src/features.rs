//! This module defines the `BackupFeature` trait, which serves as the foundation for implementing
//! various backup-related features. Each feature can hook into different stages of the backup
//! process, such as initialization, pre-processing, file processing, and post-processing.

use std::path::Path;

/// The `BackupFeature` trait defines a set of methods that can be implemented by backup features
/// to customize the backup process. Features can hook into different stages of the backup,
/// such as initialization, pre-processing, file processing, and post-processing.
///
/// This trait is designed to be thread-safe (`Sync + Send`), allowing features to be used
/// in multi-threaded environments.
#[allow(unused_variables)]
pub trait BackupFeature: Sync + Send {
    /// Initializes the feature. This method is called once at the start of the backup process.
    /// It can be used to set up resources or dependencies required by the feature.
    ///
    /// # Arguments
    /// - `features`: A slice of all available backup features. This allows features to interact
    ///              with each other during initialization.
    fn initialize(&mut self, features: &[Box<dyn BackupFeature>]) {}

    /// Performs pre-processing tasks before the backup operation begins. This method is called
    /// for each file or directory before it is processed.
    ///
    /// # Arguments
    /// - `src`: The source path of the file or directory.
    /// - `dest`: The destination path where the file or directory will be backed up.
    /// - `features`: A slice of all available backup features.
    ///
    /// # Returns
    /// - `Ok(())`: If pre-processing succeeds.
    /// - `Err(crate::BackupError)`: If an error occurs during pre-processing.
    fn pre_process(
        &self,
        src: &Path,
        dest: &Path,
        features: &[Box<dyn BackupFeature>],
    ) -> Result<(), crate::BackupError> {
        Ok(())
    }

    /// Processes a file or directory during the backup operation. This method is called
    /// for each file or directory and can be used to implement custom logic, such as
    /// filtering, transformation, or dry-run simulation.
    ///
    /// # Arguments
    /// - `src`: The source path of the file or directory.
    /// - `dest`: The destination path where the file or directory will be backed up.
    /// - `is_dir`: A boolean indicating whether the source is a directory.
    /// - `features`: A slice of all available backup features.
    ///
    /// # Returns
    /// - `Ok(true)`: If the file or directory should be processed by subsequent features.
    /// - `Ok(false)`: If the file or directory should be skipped by subsequent features.
    /// - `Err(crate::BackupError)`: If an error occurs during processing.
    fn process_file(
        &self,
        src: &Path,
        dest: &Path,
        is_dir: bool,
        features: &[Box<dyn BackupFeature>],
    ) -> Result<bool, crate::BackupError> {
        Ok(true)
    }

    /// Performs post-processing tasks after the backup operation is complete. This method is called
    /// for each file or directory after it has been processed.
    ///
    /// # Arguments
    /// - `src`: The source path of the file or directory.
    /// - `dest`: The destination path where the file or directory was backed up.
    /// - `features`: A slice of all available backup features.
    ///
    /// # Returns
    /// - `Ok(())`: If post-processing succeeds.
    /// - `Err(crate::BackupError)`: If an error occurs during post-processing.
    fn post_process(
        &self,
        src: &Path,
        dest: &Path,
        features: &[Box<dyn BackupFeature>],
    ) -> Result<(), crate::BackupError> {
        Ok(())
    }
}
