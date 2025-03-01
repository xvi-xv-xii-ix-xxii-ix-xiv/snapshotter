//! This module provides functions to check file and directory permissions for backup operations.
//! It supports both Unix and Windows systems and includes caching to optimize repeated checks.

use std::collections::HashMap;
use std::fmt;
use std::fs;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::BackupError;

/// A type alias for a thread-safe cache of permission check results.
/// The cache maps file paths to the results of permission checks.
type PermissionCache = Arc<Mutex<HashMap<PathBuf, Arc<Result<(), BackupError>>>>>;

/// An enumeration representing different types of permissions.
#[derive(Debug, PartialEq)]
enum PermissionType {
    Read,
    Write,
    Execute,
}

impl fmt::Display for PermissionType {
    /// Implements the `Display` trait for `PermissionType` to provide human-readable descriptions.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PermissionType::Read => write!(f, "read"),
            PermissionType::Write => write!(f, "write"),
            PermissionType::Execute => write!(f, "execute"),
        }
    }
}

// A thread-safe, lazily initialized cache for storing permission check results.
lazy_static::lazy_static! {
    static ref PERMISSION_CACHE: PermissionCache = Arc::new(Mutex::new(HashMap::new()));
}

/// Checks the permissions of the source directory or file.
/// This function ensures that the source path exists and has the necessary read and execute permissions.
///
/// # Arguments
/// - `source_path`: The path to the source directory or file.
///
/// # Returns
/// - `Ok(())`: If the source path has the required permissions.
/// - `Err(BackupError)`: If the source path does not exist or lacks the required permissions.
pub fn check_source_permissions(source_path: &Path) -> Result<(), BackupError> {
    if !source_path.exists() {
        return Err(BackupError::SourceNotFound(
            source_path.to_string_lossy().into_owned(),
        ));
    }

    #[cfg(unix)]
    {
        match fs::metadata(source_path) {
            Ok(metadata) => {
                let permissions = metadata.permissions().mode();
                if permissions & 0o400 == 0 {
                    // No read permission
                    Err(BackupError::PermissionDenied(format!(
                        "No {} permission for source directory '{}'",
                        PermissionType::Read,
                        source_path.display()
                    )))
                } else if permissions & 0o100 == 0 {
                    // No execute permission
                    Err(BackupError::PermissionDenied(format!(
                        "No {} permission for source directory '{}'",
                        PermissionType::Execute,
                        source_path.display()
                    )))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(BackupError::Io(e)),
        }
    }
    #[cfg(windows)]
    {
        match fs::read_dir(source_path) {
            Ok(_) => Ok(()), // Success, permissions are valid
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                Err(BackupError::PermissionDenied(format!(
                    "No {} permission for source directory '{}'",
                    PermissionType::Read,
                    source_path.display()
                )))
            }
            Err(e) => Err(BackupError::Io(e)),
        }
    }
}

/// Checks the permissions of the target directory or file.
/// This function ensures that the target path has the necessary write and execute permissions.
/// If the target path does not exist, it checks the permissions of the parent directory.
///
/// # Arguments
/// - `target_path`: The path to the target directory or file.
///
/// # Returns
/// - `Ok(())`: If the target path (or its parent) has the required permissions.
/// - `Err(BackupError)`: If the target path lacks the required permissions.
pub fn check_target_permissions(target_path: &Path) -> Result<(), BackupError> {
    if target_path.exists() {
        #[cfg(unix)]
        {
            match fs::metadata(target_path) {
                Ok(metadata) => {
                    let permissions = metadata.permissions().mode();
                    if permissions & 0o200 == 0 {
                        // No write permission
                        Err(BackupError::PermissionDenied(format!(
                            "No {} permission for target directory '{}'",
                            PermissionType::Write,
                            target_path.display()
                        )))
                    } else if permissions & 0o100 == 0 {
                        // No execute permission
                        Err(BackupError::PermissionDenied(format!(
                            "No {} permission for target directory '{}'",
                            PermissionType::Execute,
                            target_path.display()
                        )))
                    } else {
                        Ok(())
                    }
                }
                Err(e) => Err(BackupError::Io(e)),
            }
        }
        #[cfg(windows)]
        {
            let test_file = target_path.join("test_permissions.tmp");
            match File::create(&test_file) {
                Ok(_) => {
                    fs::remove_file(&test_file)?; // Remove the test file
                    Ok(())
                }
                Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                    Err(BackupError::PermissionDenied(format!(
                        "No {} permission for target directory '{}'",
                        PermissionType::Write,
                        target_path.display()
                    )))
                }
                Err(e) => Err(BackupError::Io(e)),
            }
        }
    } else {
        // If the target directory does not exist, check the parent directory
        if let Some(parent) = target_path.parent() {
            #[cfg(unix)]
            {
                match fs::metadata(parent) {
                    Ok(metadata) => {
                        let permissions = metadata.permissions().mode();
                        if permissions & 0o200 == 0 {
                            // No write permission
                            Err(BackupError::PermissionDenied(format!(
                                "No {} permission for parent directory '{}'",
                                PermissionType::Write,
                                parent.display()
                            )))
                        } else if permissions & 0o100 == 0 {
                            // No execute permission
                            Err(BackupError::PermissionDenied(format!(
                                "No {} permission for parent directory '{}'",
                                PermissionType::Execute,
                                parent.display()
                            )))
                        } else {
                            Ok(())
                        }
                    }
                    Err(e) => Err(BackupError::Io(e)),
                }
            }
            #[cfg(windows)]
            {
                let test_file = parent.join("test_permissions.tmp");
                match File::create(&test_file) {
                    Ok(_) => {
                        fs::remove_file(&test_file)?;
                        Ok(())
                    }
                    Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                        Err(BackupError::PermissionDenied(format!(
                            "No {} permission for parent directory '{}'",
                            PermissionType::Write,
                            parent.display()
                        )))
                    }
                    Err(e) => Err(BackupError::Io(e)),
                }
            }
        } else {
            Ok(())
        }
    }
}
