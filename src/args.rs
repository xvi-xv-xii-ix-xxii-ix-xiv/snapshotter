use clap::{ArgAction, Parser};

/// Backup Utility: A tool for creating backups of directories with various features.
///
/// This utility allows you to back up directories with options for compression,
/// incremental backups, integrity verification, and more. It is highly configurable
/// and supports parallel processing for improved performance.
///
/// # Examples
///
/// Basic usage:
/// ```bash
/// snapshotter /path/to/source /path/to/target --compress --verify
/// ```
///
/// Incremental backup with 4 threads:
/// ```bash
/// snapshotter /path/to/source /path/to/target --incremental --threads 4
/// ```
///
/// Dry run (simulate backup without making changes):
/// ```bash
/// snapshotter /path/to/source /path/to/target --dry-run
/// ```
#[derive(Parser, Debug)]
#[command(
    author = "XVI.XV.XII.IX.XXII.IX.XIV",
    version = "1.2.0",
    about = "A backup utility with configurable features",
    long_about = "A tool for creating backups of directories with options for compression, incremental backups, integrity verification, and more."
)]
pub struct Args {
    /// Source directory to back up.
    ///
    /// This is the directory that will be backed up. All files and subdirectories
    /// within this directory will be included in the backup, unless excluded by
    /// the configuration file.
    #[arg(required = true)]
    pub source_dir: String,

    /// Target directory for the backup.
    ///
    /// This is the directory where the backup will be stored. A new subdirectory
    /// with a timestamp will be created here to store the backup.
    #[arg(required = true)]
    pub target_dir: String,

    /// Enable compression of the backup (creates a .tar.gz file).
    ///
    /// If enabled, the backup will be compressed into a `.tar.gz` archive.
    /// This is useful for saving disk space and reducing backup size.
    #[arg(long, action = ArgAction::SetTrue)]
    pub compress: bool,

    /// Perform an incremental backup (only copy newer files).
    ///
    /// If enabled, only files that have been modified since the last backup
    /// will be copied. This can significantly reduce backup time and storage usage.
    #[arg(long, action = ArgAction::SetTrue)]
    pub incremental: bool,

    /// Perform a dry run (simulate without making changes).
    ///
    /// If enabled, the backup process will be simulated without actually copying
    /// any files. This is useful for testing and verifying the backup configuration.
    #[arg(long, action = ArgAction::SetTrue)]
    pub dry_run: bool,

    /// Verify the integrity of the backup.
    ///
    /// If enabled, the backup will be verified after completion to ensure that
    /// all files were copied correctly and no data was corrupted.
    #[arg(long, action = ArgAction::SetTrue)]
    pub verify: bool,

    /// Number of threads to use for parallel processing.
    ///
    /// This controls the number of threads used for parallel file copying.
    /// By default, the number of CPU cores is used. Set this to a lower value
    /// to reduce CPU usage or to a higher value for faster backups (if supported
    /// by your hardware).
    #[arg(long, default_value_t = rayon::current_num_threads())]
    pub threads: usize,
}

impl Args {
    /// Parse command-line arguments and validate them.
    ///
    /// This function parses the command-line arguments using `clap` and performs
    /// basic validation to ensure that the provided values are valid.
    ///
    /// # Returns
    /// - `Ok(Self)`: If the arguments are valid.
    /// - `Err(BackupError)`: If the arguments are invalid (e.g., invalid number of threads).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use snapshotter::args::Args;
    ///
    /// let args = Args::parse_and_validate().unwrap();
    /// println!("Source directory: {}", args.source_dir);
    /// ```
    pub fn parse_and_validate() -> Result<Self, crate::BackupError> {
        let args = Args::parse();

        // Validate the number of threads
        if args.threads == 0 {
            return Err(crate::BackupError::InvalidThreads(
                "Number of threads must be greater than 0".to_string(),
            ));
        }

        Ok(args)
    }
}
