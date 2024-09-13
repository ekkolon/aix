use crate::project::extras::ProjectExtra;
use crate::{handle::AppHandle, utils::interpolation::replace_template_vars_all};
use serde_json::json;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tokio::fs;

use crate::project::extras;
use crate::project::extras::ci::CI;

/// A builder for constructing `Project` instances.
#[derive(Clone)]
pub struct ProjectBuilder {
    app_handle: AppHandle,
    project: Project,
}

impl ProjectBuilder {
    /// Creates a new `ProjectBuilder` with a default `Project` instance.
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            project: Project::default(),
        }
    }

    /// Sets the name of the project.
    pub fn name(mut self, name: &str) -> Self {
        self.project.name = name.into();
        self
    }

    /// Sets the type of the project (`Standalone` or `Workspace`).
    pub fn typ(mut self, typ: ProjectType) -> Self {
        self.project.typ = typ;
        self
    }

    /// Sets the root directory of the project's source code.
    pub fn src_root<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.project.src_root = dir.as_ref().to_path_buf();
        self
    }

    /// Configures the project to include CI setup based on the specified boolean flag.
    pub fn setup_ci(mut self, yes: bool) -> Self {
        if yes && !self.project.has_extra(&ProjectExtra::CI) {
            self.project.add_extra(ProjectExtra::CI);
            return self;
        }
        self.project.remove_extra(&ProjectExtra::CI);
        self
    }

    /// Configures the project to include Docker setup based on the specified boolean flag.
    pub fn setup_docker(mut self, yes: bool) -> Self {
        if yes && !self.project.has_extra(&ProjectExtra::Docker) {
            self.project.add_extra(ProjectExtra::Docker);
            return self;
        }
        self.project.remove_extra(&ProjectExtra::Docker);
        self
    }

    /// Finalizes the construction of the `Project` instance, sets up project templates,
    /// and applies additional configurations (e.g ci, docker).
    pub async fn build(self) -> crate::Result<Project> {
        let project_typ = self.project.typ();

        let template_dir = get_template_dir(&self.app_handle, project_typ);
        if !fs::try_exists(&template_dir).await? {
            // Pull project templates from repository.
            self.app_handle.sync_store().await?;
        }

        // Copy templates for this project type to target src dir.
        let src_root = &self.project.src_root();
        let project_name = &self.project.name();
        crate::fs::copy_dir_all(&template_dir, src_root).await?;

        if *project_typ == ProjectType::Workspace {
            // The workspace template uses the standalone template to scaffold
            // the initial crate member.
            let crate_path = src_root.join(project_name);
            let template_dir = get_template_dir(&self.app_handle, &ProjectType::Standalone);
            crate::fs::copy_dir_all(&template_dir, crate_path).await?;
        }

        if self.project.has_extra(&ProjectExtra::Docker) {
            extras::docker::setup_docker(&self.app_handle, &self.project).await?;
        }

        if self.project.has_extra(&ProjectExtra::CI) {
            extras::ci::setup_ci(&self.app_handle, &self.project, &CI::GitHub).await?;
        }

        Ok(self.project)
    }
}

/// Represents the type of a project.
///
/// # Variants
///
/// - `Standalone`: Indicates a standalone project.
/// - `Workspace`: Indicates a workspace project, typically containing multiple projects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectType {
    /// Indicates a standalone project.
    #[default]
    Standalone,
    // Indicates a workspace project, typically containing multiple projects.
    Workspace,
}

impl Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standalone => write!(f, "standalone"),
            Self::Workspace => write!(f, "workspace"),
        }
    }
}

impl FromStr for ProjectType {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standalone" => Ok(ProjectType::Standalone),
            "workspace" => Ok(Self::Workspace),
            _ => Err(crate::Error::InvalidProjectKind { kind: s.into() }),
        }
    }
}

/// Represents a project with associated metadata and configuration.
#[derive(Clone, Default)]
pub struct Project {
    pub name: String,
    pub typ: ProjectType,
    pub src_root: PathBuf,
    extras: Vec<ProjectExtra>,
}

impl Project {
    /// Creates a new `Project` instance with specified parameters.
    pub async fn new<P>(
        typ: ProjectType,
        name: &str,
        src_root: P,
        extras: &[ProjectExtra],
    ) -> crate::Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(Self {
            typ,
            name: name.into(),
            src_root: src_root.as_ref().to_path_buf(),
            extras: extras.to_vec(),
        })
    }

    /// Gets the type of the project.
    pub fn typ(&self) -> &ProjectType {
        &self.typ
    }

    /// Gets the root directory of the project's source code.
    pub fn src_root(&self) -> &Path {
        &self.src_root
    }

    /// Gets the name of the project.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the list of extra configurations.
    pub fn extras(&self) -> &Vec<ProjectExtra> {
        &self.extras
    }

    /// Checks if a specific extra configuration is present.
    pub fn has_extra(&self, extra: &ProjectExtra) -> bool {
        self.extras.contains(extra)
    }

    /// Replaces the list of extra configurations.
    pub fn set_extras(&mut self, extras: Vec<ProjectExtra>) {
        self.extras = extras;
    }

    /// Adds a new extra configuration to the list.
    pub fn add_extra(&mut self, extra: ProjectExtra) {
        self.extras.push(extra);
    }

    /// Removes a specific extra configuration from the list if present.
    pub fn remove_extra(&mut self, extra: &ProjectExtra) -> bool {
        if let Some(index) = self.extras.iter().position(|e| e == extra) {
            self.extras.remove(index);
            return true;
        }
        false
    }

    /// Compiles the project by replacing template variables in the source code.
    pub async fn compile(&self) -> crate::Result<Vec<String>> {
        let vars = json!({
            "crate_name": &self.name,
            "rust_version": "1.75"
        });

        let files = replace_template_vars_all(&self.src_root, vec![], vars).await?;
        Ok(files)
    }
}

/// Retrieves the directory path for project templates based on the project type.
///
/// # Arguments
///
/// - `app_handle`: An instance of `AppHandle` used to access application configuration.
/// - `project_type`: The type of the project for which to get the template directory.
///
/// # Returns
///
/// Returns a `PathBuf` representing the path to the directory containing templates for
/// the specified project type.
fn get_template_dir(app_handle: &AppHandle, project_type: &ProjectType) -> PathBuf {
    app_handle
        .templates_dir()
        .to_path_buf()
        .join(project_type.to_string())
}
