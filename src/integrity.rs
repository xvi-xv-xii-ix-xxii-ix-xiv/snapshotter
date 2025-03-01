//! This module provides the `IntegrityFeature` struct, which is used to verify the integrity
//! of files after they have been backed up. It ensures that the source and destination files
//! are identical by comparing their SHA-256 checksums.

use crate::features::BackupFeature;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

/// The `IntegrityFeature` struct is used to enable or disable file integrity verification.
/// When enabled, it calculates and compares the SHA-256 checksums of the source and destination
/// files to ensure they are identical.
///
/// # Fields
/// - `enabled`: A boolean flag that determines whether integrity verification is active.
pub struct IntegrityFeature {
    pub enabled: bool,
}

impl BackupFeature for IntegrityFeature {
    /// Performs an integrity check after the backup operation is complete. This method calculates
    /// the SHA-256 checksums of the source and destination files and compares them to ensure
    /// they are identical. If the checksums do not match, an error is returned.
    ///
    /// # Arguments
    /// - `src`: The source path of the file.
    /// - `dest`: The destination path where the file was backed up.
    /// - `_features`: A slice of additional backup features (unused in this implementation).
    ///
    /// # Returns
    /// - `Ok(())`: If the integrity check passes (i.e., the files are identical).
    /// - `Err(crate::BackupError)`: If the integrity check fails or an error occurs during file reading.
    fn post_process(
        &self,
        src: &Path,
        dest: &Path,
        _features: &[Box<dyn BackupFeature>],
    ) -> Result<(), crate::BackupError> {
        // Skip integrity checks if the feature is disabled or the source is a directory.
        if self.enabled && !src.is_dir() {
            // Initialize SHA-256 hashers for the source and destination files.
            let mut src_hasher = Sha256::new();
            let mut dest_hasher = Sha256::new();

            // Open the source and destination files for reading.
            let mut src_file = BufReader::new(File::open(src)?);
            let mut dest_file = BufReader::new(File::open(dest)?);

            // Buffer for reading file data in chunks.
            let mut buffer = [0; 4096];

            // Read and hash the files in chunks.
            loop {
                // Read a chunk from the source file.
                let n = src_file.read(&mut buffer)?;
                if n == 0 {
                    break; // End of file reached.
                }
                // Update the source file's hash with the chunk.
                src_hasher.update(&buffer[..n]);

                // Read a chunk from the destination file.
                let n_dest = dest_file.read(&mut buffer)?;
                if n_dest != n {
                    // If the chunk sizes differ, the files are not identical.
                    return Err(io::Error::new(io::ErrorKind::Other, "File size mismatch").into());
                }
                // Update the destination file's hash with the chunk.
                dest_hasher.update(&buffer[..n]);
            }

            // Finalize the hashes.
            let src_hash = src_hasher.finalize();
            let dest_hash = dest_hasher.finalize();

            // Compare the hashes.
            if src_hash != dest_hash {
                // If the hashes differ, the files are not identical.
                return Err(io::Error::new(io::ErrorKind::Other, "Integrity check failed").into());
            }
        }

        // If the integrity check passes, return success.
        Ok(())
    }
}
