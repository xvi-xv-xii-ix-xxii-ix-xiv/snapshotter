use crate::features::BackupFeature;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Checks if a given path is a symbolic link.
///
/// # Arguments
/// - `path`: The path to check.
///
/// # Returns
/// - `true` if the path is a symbolic link.
/// - `false` otherwise.
fn is_symlink(path: &Path) -> bool {
    if let Ok(metadata) = fs::symlink_metadata(path) {
        metadata.file_type().is_symlink()
    } else {
        false
    }
}

/// Normalizes a path by converting it to its canonical form.
///
/// # Arguments
/// - `path`: The path to normalize.
///
/// # Returns
/// - The normalized `PathBuf`.
fn normalize_path(path: &Path) -> PathBuf {
    if let Ok(canonical_path) = fs::canonicalize(path) {
        canonical_path
    } else {
        path.to_path_buf()
    }
}

/// Copies a file from the source path to the destination path.
///
/// # Arguments
/// - `src`: The source file path.
/// - `dest`: The destination file path.
///
/// # Returns
/// - `Ok(())` if the file is copied successfully.
/// - `Err(BackupError)` if an error occurs during the copy operation.
fn copy_file(src: &Path, dest: &Path) -> Result<(), crate::BackupError> {
    let src_file = File::open(src)?;
    let dest_file = File::create(dest)?;
    let mut reader = BufReader::new(src_file);
    let mut writer = BufWriter::new(dest_file);
    io::copy(&mut reader, &mut writer)?;
    Ok(())
}

/// Processes a directory for backup, copying files and subdirectories.
///
/// # Arguments
/// - `src_path`: The source directory path.
/// - `dest_path`: The destination directory path.
/// - `skip_folders_and_files`: A list of folder and file names to skip.
/// - `skip_file_extensions`: A list of file extensions to skip.
/// - `features`: A list of backup features to apply.
/// - `processed_dirs`: A shared set of already processed directories to avoid cycles.
///
/// # Returns
/// - `Ok(Vec<(PathBuf, PathBuf)>)`: A list of new directories to process.
/// - `Err(BackupError)`: If an error occurs during processing.
fn process_directory(
    src_path: &Path,
    dest_path: &Path,
    _skip_folders_and_files: &[String],
    _skip_file_extensions: &[String],
    features: &[Box<dyn BackupFeature>],
    processed_dirs: Arc<Mutex<HashSet<PathBuf>>>,
) -> Result<Vec<(PathBuf, PathBuf)>, crate::BackupError> {
    let src_path_normalized = normalize_path(src_path);

    {
        let mut processed = processed_dirs.lock().unwrap();
        if !processed.insert(src_path_normalized.clone()) {
            return Ok(Vec::new());
        }
    }

    // Apply pre-processing features
    for feature in features {
        feature.pre_process(src_path, dest_path, features)?;
    }

    // Create the destination directory
    fs::create_dir_all(dest_path)?;

    // Read the source directory entries
    let entries: Vec<_> = fs::read_dir(src_path)?.collect::<Result<_, _>>()?;

    // Process entries in parallel
    let new_dirs: Vec<(PathBuf, PathBuf)> = entries
        .par_iter()
        .filter_map(|entry| {
            let file_name = entry.file_name();
            let src_item = entry.path();
            let dest_item = dest_path.join(&file_name);

            // Check if the file should be processed based on features
            let mut should_process = true;
            for feature in features {
                match feature.process_file(&src_item, &dest_item, src_item.is_dir(), features) {
                    Ok(val) => {
                        should_process = val;
                        if !should_process {
                            break;
                        }
                    }
                    Err(e) => return Some(Err(e)),
                }
            }

            if !should_process {
                return None;
            }

            if src_item.is_dir() {
                // Skip symbolic links to avoid cycles
                if is_symlink(&src_item) {
                    println!("Skipping symlink: {}", src_item.display());
                    return None;
                }
                Some(Ok((src_item, dest_item)))
            } else {
                // Copy the file
                match copy_file(&src_item, &dest_item) {
                    Ok(_) => None,
                    Err(e) => Some(Err(e)),
                }
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Apply post-processing features
    for feature in features {
        feature.post_process(src_path, dest_path, features)?;
    }

    Ok(new_dirs)
}

/// Copies a directory recursively with parallel processing.
///
/// # Arguments
/// - `src`: The source directory path.
/// - `dest`: The destination directory path.
/// - `skip_folders_and_files`: A list of folder and file names to skip.
/// - `skip_file_extensions`: A list of file extensions to skip.
/// - `features`: A list of backup features to apply.
/// - `num_threads`: The number of threads to use for parallel processing.
///
/// # Returns
/// - `Ok(())` if the directory is copied successfully.
/// - `Err(BackupError)` if an error occurs during the copy operation.
pub fn copy_directory(
    src: &Path,
    dest: &Path,
    skip_folders_and_files: &[String],
    skip_file_extensions: &[String],
    features: &[Box<dyn BackupFeature>],
    num_threads: usize,
) -> Result<(), crate::BackupError> {
    // Create a thread pool with the specified number of threads
    let pool = ThreadPoolBuilder::new().num_threads(num_threads).build()?;

    // Track processed directories to avoid cycles
    let processed_dirs = Arc::new(Mutex::new(HashSet::new()));

    // Initialize the task queue with the source and destination paths
    let mut tasks = vec![(normalize_path(src), normalize_path(dest))];

    // Process directories in parallel
    while !tasks.is_empty() {
        let new_tasks: Vec<(PathBuf, PathBuf)> = pool.install(|| {
            tasks
                .par_iter()
                .flat_map(|(src_path, dest_path)| {
                    process_directory(
                        src_path,
                        dest_path,
                        skip_folders_and_files,
                        skip_file_extensions,
                        features,
                        processed_dirs.clone(),
                    )
                    .unwrap_or_else(|e| {
                        eprintln!("Error processing {}: {}", src_path.display(), e);
                        Vec::new()
                    })
                })
                .collect()
        });

        tasks = new_tasks;
    }

    Ok(())
}
