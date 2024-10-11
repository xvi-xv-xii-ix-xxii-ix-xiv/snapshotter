use std::fs;
use std::path::Path;

/// Recursively copies a directory while excluding specific items and file extensions.
///
/// # Parameters
/// - `src`: Source directory path to be copied.
/// - `dest`: Destination directory where the contents will be copied.
/// - `exclude_list`: A list of items (files or directories) to exclude from copying.
/// - `exclude_extensions`: A list of file extensions to exclude from copying.
///
/// # Returns
/// - `std::io::Result<()>`: Returns an empty `Ok(())` if successful.
///
/// # Errors
/// - Will return an error if any file or directory operations fail during copying.
///
/// # Panics
/// - The function does not panic unless the caller unwraps a potential error result.
pub fn copy_directory(
    src: &Path,
    dest: &Path,
    exclude_list: &[String],
    exclude_extensions: &[String],
) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let src_item = entry.path();

        // Skip excluded directories or files based on the exclude list
        let file_name_str = file_name.to_string_lossy().to_string();
        if exclude_list.contains(&file_name_str) {
            continue; // Skip this item, as it's in the exclude list
        }

        let dest_item = dest.join(&file_name);

        if src_item.is_dir() {
            // Recursively copy directories
            fs::create_dir_all(&dest_item)?;
            copy_directory(&src_item, &dest_item, exclude_list, exclude_extensions)?;
        } else {
            // Check file extension and skip if it's in the exclude_extensions list
            if let Some(extension) = src_item.extension() {
                let extension_str = extension.to_string_lossy().to_string();
                if exclude_extensions.contains(&extension_str) {
                    // Skip the file if its extension is in the exclude list
                    continue;
                }
            }

            // If the file extension is not in the exclude list, copy the file
            fs::copy(&src_item, &dest_item)?;
        }
    }
    Ok(())
}
