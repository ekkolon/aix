// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

use async_recursion::async_recursion;
use std::path::Path;
use tokio::fs;

/// Recreates a directory at the specified `path`, ensuring it is empty.
///
/// # Warnings
/// **Be careful:** This function will remove **all** contents in the specified directory if it exists.
///
/// # Arguments
/// - `path`: The path to the directory that needs to be recreated.
///
/// # Errors
/// - Returns `crate::Error::NotADirectory` if `path` exists but is not a directory.
/// - Propagates I/O errors from directory deletion or creation.
pub async fn recreate_dir<P: AsRef<Path>>(path: P) -> crate::Result<()> {
    let path = path.as_ref();
    if path.exists() {
        if !path.is_dir() {
            return Err(crate::Error::NotADirectory {
                path: path.to_path_buf(),
            });
        }

        fs::remove_dir_all(path).await?;
    }

    fs::create_dir_all(path).await?;
    Ok(())
}

/// Recursively copies the contents of a source directory (`src`) to a destination directory (`dest`).
/// 
/// # Arguments
/// - `src`: The path of the source directory to copy from.
/// - `dest`: The path of the destination directory to copy to.
///
/// # Errors
/// - Propagates any I/O errors encountered while reading the source directory or copying files.
///
/// # Example
/// ```rust
/// copy_dir_all("src_folder", "dest_folder").await?;
/// ```
#[async_recursion]
pub async fn copy_dir_all<S, D>(src: S, dest: D) -> crate::Result<()>
where
    S: AsRef<Path> + Send + Sync,
    D: AsRef<Path> + Send + Sync,
{
    let dst_path = dest.as_ref();
    let src_path = src.as_ref();

    // Create the destination directory if it doesn't exist
    fs::create_dir_all(dst_path).await?;

    // Read the source directory entries
    let mut entries = fs::read_dir(src_path).await?;

    // Process each entry
    while let Some(entry) = entries.next_entry().await? {
        let file_type = entry.file_type().await?;
        let entry_dst = dst_path.join(entry.file_name());

        // Recursively copy directories or copy files
        if file_type.is_dir() {
            copy_dir_all(entry.path(), entry_dst).await?;
        } else {
            fs::copy(entry.path(), entry_dst).await?;
        }
    }

    Ok(())
}

/// Removes all files and directories inside the specified directory (`dir`), except for those listed in `keep_paths`.
///
/// # Arguments
/// - `dir`: The path to the directory where files/directories will be removed.
/// - `keep_paths`: A list of file or directory names (strings) to keep.
///
/// # Errors
/// - Propagates any I/O errors encountered during file or directory removal.
///
/// # Example
/// ```no_run
/// remove_files_except("target_dir", &["keep_me.txt", "important_dir"]).await?;
/// ```
pub async fn remove_files_except<D>(dir: D, keep_paths: &[&str]) -> crate::Result<()>
where
    D: AsRef<Path>,
{
    let mut entries = fs::read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let entry_name = path.file_name().unwrap().to_str().unwrap();

        if !keep_paths.contains(&entry_name) {
            if path.is_file() {
                fs::remove_file(&path).await?;
            } else if path.is_dir() {
                fs::remove_dir_all(&path).await?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

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
}
