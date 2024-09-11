use std::path::{Path, PathBuf};

use tokio::fs;

#[derive(Clone)]
pub struct Context {
    app_name: String,
    local_data_dir: PathBuf,
    templates_dir: PathBuf,
    tmp_dir: PathBuf,
}

pub const RELATIVE_TEMPLATES_DIR: &str = "templates";
pub const RELATIVE_TEMP_DIR: &str = ".tmp";

impl Context {
    pub fn new(app_name: &str) -> crate::Result<Self> {
        let local_data_dir = dirs::data_local_dir().ok_or(crate::Error::MissingLocalDataDir)?;
        let templates_dir = local_data_dir.join(app_name).join(RELATIVE_TEMPLATES_DIR);
        let tmp_dir = local_data_dir.join(app_name).join(RELATIVE_TEMP_DIR);

        Ok(Self {
            app_name: app_name.into(),
            local_data_dir,
            templates_dir,
            tmp_dir,
        })
    }

    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    pub fn local_data_dir(&self) -> &Path {
        &self.local_data_dir
    }

    pub fn templates_dir(&self) -> &Path {
        &self.templates_dir
    }

    pub fn temp_dir(&self) -> &Path {
        &self.tmp_dir
    }

    pub async fn ensure_templates_dir(&self) -> crate::Result<&Path> {
        fs::create_dir_all(&self.templates_dir).await?;
        Ok(&self.templates_dir)
    }

    pub async fn ensure_tmp_dir<P: AsRef<Path>>(&self, path: P) -> crate::Result<PathBuf> {
        let tmp_dir = &self.tmp_dir.join(path.as_ref());
        fs::create_dir_all(&tmp_dir).await?;
        Ok(tmp_dir.into())
    }

    pub async fn fetch_local<P: AsRef<Path>>(&self, path: P) -> crate::Result<PathBuf> {
        let tmp_dir = &self.tmp_dir.join(path.as_ref());
        fs::create_dir_all(&tmp_dir).await?;
        Ok(tmp_dir.into())
    }
}
