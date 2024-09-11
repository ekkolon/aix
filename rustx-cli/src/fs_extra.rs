use async_recursion::async_recursion;
use std::path::Path;
use tokio::fs;

#[async_recursion]
pub async fn copy_dir_all<S, D>(src: S, dest: D) -> Result<(), std::io::Error>
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

pub async fn remove_files_except<D>(dir: D, keep_paths: &[&str]) -> crate::Result<()>
where
    D: AsRef<Path>,
{
    let mut entries = fs::read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let entry_name = path.file_name().unwrap().to_str().unwrap();

        // Check if the entry is in the keep_paths list (files or directories)
        if !keep_paths.contains(&entry_name) {
            if path.is_file() {
                // Remove the file if it's not in the keep list
                fs::remove_file(&path).await?;
                println!("Removed file: {:?}", entry_name);
            } else if path.is_dir() {
                // Recursively remove the directory and its contents if it's not in the keep list
                fs::remove_dir_all(&path).await?;
                println!("Removed directory: {:?}", entry_name);
            }
        }
    }

    Ok(())
}
