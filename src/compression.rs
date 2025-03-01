use crate::features::BackupFeature;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::{self, File};
use std::path::Path;
use tar::Builder;

/// A feature that compresses the backup directory into a `.tar.gz` archive.
///
/// This feature is enabled by the `--compress` command-line argument.
/// When enabled, it compresses the backup directory into a `.tar.gz` archive
/// and removes the original directory to save space.
///
/// # Examples
///
/// Enable compression:
/// ```bash
/// snapshotter /path/to/source /path/to/target --compress
/// ```
pub struct CompressionFeature {
    /// Whether compression is enabled.
    pub enabled: bool,
}

impl BackupFeature for CompressionFeature {
    /// Compresses the backup directory into a `.tar.gz` archive.
    ///
    /// This function is called after the backup process is complete.
    /// If compression is enabled, it creates a `.tar.gz` archive of the backup
    /// directory and removes the original directory.
    ///
    /// # Arguments
    /// - `_src`: The source directory (not used in this feature).
    /// - `dest`: The destination directory to compress.
    /// - `_features`: A list of backup features (not used in this feature).
    ///
    /// # Returns
    /// - `Ok(())` if the compression is successful.
    /// - `Err(BackupError)` if an error occurs during compression.
    fn post_process(
        &self,
        _src: &Path,
        dest: &Path,
        _features: &[Box<dyn BackupFeature>],
    ) -> Result<(), crate::BackupError> {
        if self.enabled {
            // Create a `.tar.gz` file for the backup
            let tar_gz = File::create(dest.with_extension("tar.gz"))?;
            let enc = GzEncoder::new(tar_gz, Compression::default());

            // Create a tar archive
            let mut tar = Builder::new(enc);
            tar.append_dir_all(".", dest)?;
            tar.finish()?;

            // Remove the original backup directory
            fs::remove_dir_all(dest)?;
        }
        Ok(())
    }
}
