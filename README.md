## Snapshotter
Version: 1.2.0

## Description

Snapshotter is a powerful and efficient directory backup utility written in Rust. It allows you to exclude specific files and directories as defined in a config.yaml configuration file. The tool supports multiple configuration sections (e.g., python, rust, or default), providing flexibility to customize backups for different project types. With new features like permission checks, wildcard exclusions, and enhanced logging, Snapshotter ensures reliable and customizable backups.

## Features

- Backup any directory with an automatically appended timestamp for easy versioning.
- Exclude files, folders, and file extensions using wildcards (e.g., *.log) as specified in config.yaml.
- Compress backups into .tar.gz format.
- Incremental backups to copy only new or modified files.
- Logging of operations for debugging and auditing.
- Dry run mode to preview actions without copying.
- Support for multiple source directories in a single command.
- Integrity verification of backups using SHA-256 hashes.
- Permission checks to ensure proper access rights for source and target directories.
- Wildcard-based exclusions for fine-grained control over skipped files and folders.
- Parallel directory processing with rayon for improved performance.
- Optimized file copying using buffered I/O.

## Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/xvi-xv-xii-ix-xxii-ix-xiv/snapshotter.git

## Usage

```bash
snapshotter <source_dir> <target_dir> [config_section] [--compress] [--incremental] [--dry-run] [--verify]
```

- <source_dir>: Directory to back up.
- <target_dir>: Directory where backups are created.
- [config_section]: Optional configuration section (e.g., python, rust). Defaults to default.
- --compress: Compress the backup into .tar.gz.
- --incremental: Perform an incremental backup.
- --dry-run: Simulate the backup without copying.
- --verify: Verify backup integrity with SHA-256.

Example
- Backup a Python project with compression:

```bash
./target/release/snapshotter /path/to/python_project /path/to/backup python --compress
```
- Incremental backup of multiple sources:
```bash
./target/release/snapshotter /path/to/proj1 /path/to/proj2 /path/to/backup --incremental
```

- Dry run with verification:
```bash
./target/release/snapshotter /path/to/docs /path/to/backup --dry-run --verify
```

## Configuration

The configuration file config.yaml is used to define what files and directories should be excluded from the backup.
It supports multiple sections for different environments (e.g., python, rust, default).

- The config.yaml file defines exclusions with support for wildcards.
```yaml
default:
  skip_folders_and_files:
    - "*.log"     # Exclude all log files
    - "temp-*"    # Exclude temp directories
  skip_file_extensions:
    - "pyc"
    - "cache"
```

Example config.yaml:

```yaml
# Default configuration for general backups
default:
   skip_folders_and_files:
      - .venv       # Virtual environment directory
      - venv
      - data        # Temporary data folder
      - target      # Rust build directory
      - debug       # Debug build artifacts
      - tmp         # Temporary directory
      - temp
   skip_file_extensions:
      - pyc         # Python compiled files
      - log         # Log files
      - cache       # Cache files
      - lock        # Lock files
      - rlib        # Rust library files

# Configuration for Python projects
python:
   skip_folders_and_files:
      - .venv       # Virtual environment
      - venv
      - data        # Temporary data
   skip_file_extensions:
      - pyc         # Python compiled files
      - log         # Log files
      - cache       # Cache files

# Configuration for Rust projects
rust:
   skip_folders_and_files:
      - target      # Rust build directory
      - debug       # Debug build artifacts
   skip_file_extensions:
      - rlib        # Rust library files
      - lock        # Cargo lock file
```

## Configuration Parameters:

- skip_folders_and_files: A list of folder or file names to exclude from the backup.
- skip_file_extensions: A list of file extensions (without the dot) to exclude from the backup.

If no configuration section is specified in the command, the default section is used.
Adding more sections
To support new environments, add a new section to config.yaml. For example:

```yaml
# Configuration for JavaScript projects
javascript:
skip_folders_and_files:
- node_modules  # Dependency directory
- dist          # Build output
skip_file_extensions:
- js.map        # Source maps
- log           # Log files
```

Then use it with:
```bash
./target/release/snapshotter /path/to/js_project /path/to/backup javascript
```

## New Features in v1.2.0

1. Permission Checks

- Ensures that the source directory has read and execute permissions.
Ensures that the target directory (or its parent) has write and execute permissions.
Use the --check-permissions flag to enable this feature.
2. Wildcard Exclusions

- Skip files, folders, and extensions using glob patterns (e.g., *.log, temp-*).
- Configured in config.yaml under skip_folders_and_files and skip_file_extensions.
3. Enhanced Logging

- Detailed logging of backup operations, including skipped files and permission checks.
- Logs are printed to the console with timestamps for easy debugging.
4. Integrity Verification

- Verifies backup integrity using SHA-256 hashes.
- Use the --verify flag to enable this feature.
5. Dry Run Mode

- Simulates the backup process without copying files.
- Use the --dry-run flag to preview actions.

## Performance Optimizations
- Parallel Processing: Uses rayon to copy directories concurrently, speeding up backups for large folder structures.
- Buffered I/O: Employs BufReader and BufWriter for efficient file copying, reducing system overhead.

## Contributing
Contributions are welcome! Please fork the repository, create a branch, and submit a pull request with your changes.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

License

This project is licensed under the MIT License. See the LICENSE file for details.

Author

Developed by XVI.XV.XII.IX.XXII.IX.XIV
