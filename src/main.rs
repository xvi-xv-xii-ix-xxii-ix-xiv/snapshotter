// Backup Utility
// Version: 1.0.0
//
// Author: XVI.XV.XII.IX.XXII.IX.XIV
// License: MIT License
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
//
// Modules:
// - backup: Handles directory backup operations
// - config: Manages configuration settings for the backup process
//
// Usage:
// This tool is designed to back up directories with the ability to exclude certain files
// based on items and file extensions specified in a configuration file (config.json).
//
// Example:
// $ snapshotter <source_dir> <target_dir>

mod backup;
mod config;

use chrono::Local;
use config::Config;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!(
            "Usage: {} <source_dir> <target_dir> [config_section]",
            args[0]
        );
        return;
    }

    let source_dir = &args[1];
    let target_dir = &args[2];
    let config_section = if args.len() == 4 {
        &args[3] // Use the specified section
    } else {
        "default" // Fallback to default section
    };

    // Load configuration from config.json
    let config: Config = config::load_config(config_section).expect("Failed to load configuration");

    // Create new backup directory with timestamp
    let source_dir_name = Path::new(source_dir).file_name().unwrap().to_string_lossy();
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let backup_dir_with_timestamp =
        PathBuf::from(target_dir).join(format!("{}_backup_{}", source_dir_name, timestamp));

    // Create backup directory
    fs::create_dir_all(&backup_dir_with_timestamp).expect("Failed to create backup directory");

    // Copy source directory to backup directory
    backup::copy_directory(
        Path::new(source_dir),
        &backup_dir_with_timestamp,
        &config.excluded_items,
        &config.excluded_extensions,
    )
    .expect("Error during directory copying");

    println!("Backup created at {:?}", backup_dir_with_timestamp);
}
