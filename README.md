Backup Utility

Version: 1.0.0

Description

This is a directory backup utility that allows you to exclude specific files and directories as defined in a configuration file (config.json). The tool supports multiple configuration sections, such as Python, Rust, or a default section, giving you flexibility over which files and folders to exclude during backup.

Features

Backup any directory and automatically append a timestamp to the backup folder.
Exclude files and directories as specified in the configuration file.
Support for multiple configuration sections (e.g., Python, Rust).
Easy command-line usage with the option to specify a configuration section.
Installation

1. Clone the repository
   git clone https://github.com/Yngvarrrr/snapshotter.git

2. Install dependencies
   Ensure you have Rust and Cargo installed on your system. You can install Rust using rustup.
   cd snapshotter
   cargo build --release

This will compile the utility and produce an executable in the target/release/ directory.

Usage

To use the utility, run the following command:
backup <source_dir> <target_dir> [config_section]

<source_dir>: The path to the directory you want to back up.
<target_dir>: The directory where the backup should be created.
[config_section] (optional): The configuration section to use (e.g., python, rust). If no section is specified, the default section will be used.

Example
Backup a Python project:
snapshotter /path/to/python_project /path/to/backup python

Backup using the default section:
snapshotter /path/to/important_docs /path/to/backup

Configuration

The configuration file config.json is used to define what files and directories should be excluded from the backup.
It supports multiple sections for different environments (e.g., python, rust, default).

Example config.json:
```json
{
  "default": {
    "excluded_items": [
      ".venv",
      "venv",
      "data",
      "target",
      "debug",
      "tmp",
      "temp"
    ],
    "excluded_extensions": ["pyc", "log", "cache", "lock", "rlib"]
  },
  "python": {
    "excluded_items": [".venv", "venv", "data"],
    "excluded_extensions": ["pyc", "log", "cache"]
  },
  "rust": {
    "excluded_items": ["target", "debug"],
    "excluded_extensions": ["rlib", "lock"]
  }
}
```

Each section defines two parameters:

excluded_items: A list of file or directory names to exclude from the backup.
excluded_extensions: A list of file extensions to exclude from the backup.
If no configuration section is specified in the command, the default section will be used.

Adding more sections
To support new environments, simply add new sections in the config.json file. For example:
```json
  "javascript": {
    "excluded_items": ["target", "debug"],
    "excluded_extensions": ["rlib", "lock"]
  }
```

License

This project is licensed under the MIT License. See the LICENSE file for details.

Author

Developed by XVI.XV.XII.IX.XXII.IX.XIV
