// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Recursively replaces template variables in files within a directory using provided data.
///
/// # Purpose
/// This function processes all files within a specified directory (`src`), replacing any template variables
/// with corresponding values derived from the provided data (`data`). It skips files or directories specified
/// in the `exclude` list.
///
/// # Arguments
/// - `src`: The root directory path where the search for files begins.
/// - `exclude`: A list of file or directory names to exclude from processing.
/// - `data`: Data to be used for template variable replacements. This is serialized into a JSON map for replacement.
///
/// # Errors
/// - Returns errors from JSON serialization/deserialization if the `data` cannot be converted to a JSON map.
/// - Propagates errors from the recursive file processing function or I/O operations.
pub async fn replace_template_vars_all<T, D>(
    src: T,
    exclude: Vec<String>,
    data: D,
) -> crate::Result<Vec<String>>
where
    T: AsRef<Path>,
    D: Serialize + Send + Sync + 'static,
{
    let src = src.as_ref();

    // Convert the data into a JSON map
    let json_map: HashMap<String, String> =
        serde_json::from_value(serde_json::to_value(data).unwrap()).unwrap();

    // Recursively process directories and files asynchronously
    let processed_files =
        replace_template_vars_all_recursive(src.to_path_buf(), exclude, json_map).await?;

    Ok(processed_files)
}

type WalkDirResult = Pin<Box<dyn Future<Output = crate::Result<Vec<String>>> + Send>>;

/// Recursively processes all files in a directory, replacing template variables in each file.
///
/// # Purpose
/// This function traverses the specified directory (`dir`) and all its subdirectories,
/// replacing template variables in each file with values from a provided map (`json_map`).
/// It skips files or directories listed in the `exclude` vector.
///
/// # Arguments
/// - `dir`: The root directory where the search for files begins.
/// - `exclude`: A list of file or directory names to be skipped during the traversal.
/// - `json_map`: A map of key-value pairs where the key represents a template variable, and the value is the replacement string.
///
/// # Return
/// Returns a `WalkDirResult`, which is a `Result` containing a `Vec<String>` of the paths of the processed files, or an error.
///
/// # Errors
/// - Returns I/O errors encountered while reading directories or processing files.
/// - Returns errors from the `replace_template_vars` function if the variable replacement fails.
fn replace_template_vars_all_recursive(
    dir: PathBuf,
    exclude: Vec<String>,              // Use owned data
    json_map: HashMap<String, String>, // Use owned data
) -> WalkDirResult {
    Box::pin(async move {
        let mut entries = fs::read_dir(&dir).await?;

        let mut processed_files: Vec<String> = vec![];

        // Iterate through directory entries
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let file_name = entry.file_name();
            let _file_name_str = file_name.to_string_lossy();

            // Skip excluded files or directories
            if exclude.iter().any(|ex| path.ends_with(ex)) {
                continue;
            }

            if path.is_file() {
                // Process the file
                replace_template_vars(&path, &json_map).await?;
                processed_files.push(path.display().to_string());
            } else if path.is_dir() {
                // Recursively process subdirectories
                let new_exclude = exclude.clone(); // Clone the exclude list
                let new_json_map = json_map.clone(); // Clone the json_map
                let mut files =
                    replace_template_vars_all_recursive(path, new_exclude, new_json_map).await?;
                processed_files.append(&mut files);
            }
        }

        Ok(processed_files)
    })
}

/// Asynchronously replaces template variables in a file with corresponding values from a `json_map`.
///
/// # Purpose
/// This function reads the content of a file, replaces any template variables (e.g., `{{ var }}`) found within the content
/// with their respective values from the `json_map`, and then writes the modified content back to the file.
///
/// # Arguments
/// - `path`: The path to the file to be processed.
/// - `json_map`: A map where the keys are template variable names and the values are the replacements.
///
/// # Errors
/// - Returns I/O errors if reading from or writing to the file fails.
/// - Propagates any error from the `replace_template_variables` function.
async fn replace_template_vars(
    path: &Path,
    json_map: &HashMap<String, String>,
) -> crate::Result<()> {
    // Read the file content asynchronously
    let mut file = fs::File::open(path).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;

    // Replace keys with values in the file content
    let content = interpolate_content(&content, json_map);

    // Write the modified content back to the file asynchronously
    let mut file = fs::File::create(path).await?;
    file.write_all(content.as_bytes()).await?;

    Ok(())
}

/// Replaces template variables in a string with corresponding values from a `json_map`.
///
/// # Purpose
/// This function takes a string that may contain template variables in the form of `{{ var }}`
/// and replaces them with the values from the `json_map`. If a variable is not found in the map,
/// it leaves the placeholder unchanged.
///
/// # Arguments
/// - `content`: The input string that may contain template variables.
/// - `json_map`: A map where keys are template variable names and values are their replacements.
pub fn interpolate_content(content: &str, json_map: &HashMap<String, String>) -> String {
    // Regex to match {{ var }} with optional spaces inside the curly braces
    let re = Regex::new(r"\{\{\s*(\w+)\s*\}\}").unwrap();

    // Replace all matches with the corresponding value from the json_map
    re.replace_all(content, |caps: &regex::Captures| {
        let key = &caps[1]; // Capture the variable name
        json_map
            .get(key)
            .cloned()
            .unwrap_or_else(|| format!("{{{{ {} }}}}", key)) // Replace or leave unchanged
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::*;
    use tempfile::tempdir;
    use tokio::fs::{self, File};

    #[tokio::test]
    async fn test_recreate_dir_create_new() -> crate::Result<()> {
        let temp_dir = tempdir()?;
        let new_dir = temp_dir.path().join("new_dir");

        recreate_dir(&new_dir).await?;

        assert!(new_dir.is_dir());
        Ok(())
    }

    #[tokio::test]
    async fn test_recreate_dir_remove_existing() -> crate::Result<()> {
        let temp_dir = tempdir()?;
        let existing_dir = temp_dir.path().join("existing_dir");
        fs::create_dir_all(&existing_dir).await?;

        recreate_dir(&existing_dir).await?;

        assert!(existing_dir.is_dir());
        Ok(())
    }

    #[tokio::test]
    async fn test_recreate_dir_non_dir() -> crate::Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("file.txt");
        File::create(&file_path).await?;

        let result = recreate_dir(&file_path).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_dir_all() -> crate::Result<()> {
        let src_dir = tempdir()?;
        let dest_dir = tempdir()?;
        let file_in_src = src_dir.path().join("file.txt");

        // Create a file in the source directory
        let mut file = File::create(&file_in_src).await?;
        file.write_all(b"test content").await?;

        // Copy the directory
        copy_dir_all(src_dir.path(), dest_dir.path()).await?;

        let copied_file = dest_dir.path().join("file.txt");
        assert!(copied_file.exists());

        let content = fs::read(copied_file).await?;
        assert_eq!(content, b"test content");

        Ok(())
    }

    #[tokio::test]
    async fn test_copy_empty_dir() -> crate::Result<()> {
        let src_dir = tempdir()?;
        let dest_dir = tempdir()?;

        // Copy the empty directory
        copy_dir_all(src_dir.path(), dest_dir.path()).await?;

        assert!(dest_dir.path().is_dir());
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_files_except() -> crate::Result<()> {
        let temp_dir = tempdir()?;
        let file1 = temp_dir.path().join("keep.txt");
        let file2 = temp_dir.path().join("remove.txt");

        // Create files
        File::create(&file1).await?;
        File::create(&file2).await?;

        // Remove files except `keep.txt`
        remove_files_except(temp_dir.path(), &["keep.txt"]).await?;

        assert!(file1.exists());
        assert!(!file2.exists());

        Ok(())
    }

    #[tokio::test]
    async fn test_remove_no_files() -> crate::Result<()> {
        let temp_dir = tempdir()?;

        // Remove files except no files
        remove_files_except(temp_dir.path(), &[]).await?;

        assert!(temp_dir.path().is_dir());

        Ok(())
    }

    //#[tokio::test]
    //async fn test_replace_template_vars() -> crate::Result<()> {
    //    let temp_dir = tempdir()?;
    //    let file_path = temp_dir.path().join("template.txt");
    //
    //    // Create a template file with placeholders
    //    let mut file = File::create(&file_path).await?;
    //    file.write_all(b"Hello {{ name }}!").await?;
    //
    //    // Define the JSON map
    //    let json_map = HashMap::from([("name".to_string(), "World".to_string())]);
    //
    //    // Replace the template variables
    //    replace_template_vars(&file_path, &json_map).await?;
    //
    //    // Check the updated content
    //    let content = fs::read_to_string(file_path).await?;
    //    assert_eq!(content, "Hello World!");
    //
    //    Ok(())
    //}
    #[tokio::test]
    async fn test_interpolate_content() -> crate::Result<()> {
        let content = "Hello {{ name }}!";
        let mut json_map = HashMap::new();
        json_map.insert("name".to_string(), "Alice".to_string());

        let interpolated = interpolate_content(content, &json_map);
        assert_eq!(interpolated, "Hello Alice!");

        let json_map_empty = HashMap::new();
        let interpolated_empty = interpolate_content(content, &json_map_empty);
        assert_eq!(interpolated_empty, "Hello {{ name }}!");

        Ok(())
    }

    #[tokio::test]
    async fn test_replace_template_vars_all() -> crate::Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("file.txt");

        // Create a template file with placeholders
        let mut file = File::create(&file_path).await?;
        file.write_all(b"Hello {{ name }}!").await?;

        // Define the JSON map
        let json_map = HashMap::from([("name".to_string(), "Alice".to_string())]);

        // Replace the template variables in the directory
        let processed_files = replace_template_vars_all(temp_dir.path(), vec![], json_map).await?;

        assert_eq!(processed_files.len(), 1);
        assert_eq!(processed_files[0], file_path.display().to_string());

        // Check the updated content
        let content = fs::read_to_string(file_path).await?;
        assert_eq!(content, "Hello Alice!");

        Ok(())
    }
}
