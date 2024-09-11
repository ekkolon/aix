use crate::{fs_extra, project::ProjectType, Context, Error, Result, RELATIVE_TEMP_DIR};
use std::path::{Path, PathBuf};

use tokio::fs;

pub struct TemplateEngine {
    pub git_origin: String,
    pub git_branch: String,
    pub git_directory: String,
    pub context: Context,
}
impl TemplateEngine {
    pub async fn write<P: AsRef<Path>>(
        &self,
        project_type: ProjectType,
        destination: P,
    ) -> Result<()> {
        let destination = destination.as_ref();

        let template_dir = self.get_template_dir_for_kind(project_type);
        if !fs::try_exists(&template_dir).await? {
            // Pull project templates from repository.
            self.pull().await?;
        }

        // Template directory for this project kind already exists.
        // We simply copy it the provided destination directory.
        fs_extra::copy_dir_all(&template_dir, destination).await?;
        log::info(format!(
            "Copied templates from {} to {}",
            template_dir.display(),
            destination.display()
        ));

        Ok(())
    }

    pub async fn update<P: AsRef<Path>>(&self, cwd: P) -> Result<()> {
        //self.pull(cwd)?;
        log::info("Fetched Rustix Git repostiroy");

        let templates_dir = self.context.ensure_templates_dir().await?;
        fs::remove_dir_all(templates_dir).await?;
        log::info("Cleaned templates output directory");

        crate::fs_extra::copy_dir_all(&self.git_directory, templates_dir).await?;
        log::info("Copied templates to local data directory");

        Ok(())
    }

    /// Pulls all project templates from the GitHub repository specified in self
    /// and copies them to the user's local data directory of the binary.
    pub async fn pull(&self) -> Result<()> {
        if !crate::git::is_git_installed()? {
            return Err(Error::MissingGitInstallation);
        }

        let local_data_dir = self.context.local_data_dir();
        let temp_dir = self.context.temp_dir();
        crate::git::clone(local_data_dir, &self.git_origin, Some(RELATIVE_TEMP_DIR))?;
        crate::git::sparse_checkout_init_cone(temp_dir)?;
        crate::git::sparse_checkout_set_path(temp_dir, &self.git_directory)?;

        // Copy template files
        fs::remove_dir_all(self.context.templates_dir()).await?;
        let templates_dir = self.context.ensure_templates_dir().await?;
        fs_extra::remove_files_except(temp_dir, &["templates"]).await?;
        fs_extra::copy_dir_all(templates_dir, temp_dir).await?;

        Ok(())
    }

    fn cleanup_pull<P: AsRef<Path>>(&self, cwd: P) {}

    fn get_template_dir_for_kind(&self, project_kind: ProjectType) -> PathBuf {
        self.context
            .templates_dir()
            .to_path_buf()
            .join(project_kind.to_string())
    }
}

mod log {
    use log::info;
    pub fn info<T: AsRef<str>>(message: T) {
        info!("[rustx] --- {}", message.as_ref());
    }
}
