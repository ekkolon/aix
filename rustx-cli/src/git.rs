use crate::Result;
use std::{path::Path, process::Command};

pub fn is_git_installed() -> Result<bool> {
    let child = Command::new("git").arg("--version").spawn();
    if child.is_err() {
        return Ok(false);
    }

    let installed = child.unwrap().wait().map(|_| true).unwrap_or(false);
    Ok(installed)
}

pub fn clone<P, D>(current_dir: P, origin: &str, destination: Option<D>) -> Result<()>
where
    P: AsRef<Path>,
    D: AsRef<str>,
{
    let mut child = Command::new("git");

    let mut cmd = child
        .current_dir(current_dir)
        .arg("clone")
        .arg("--filter=blob:none")
        .arg(format!("--sparse {}", origin));

    if let Some(dest) = destination {
        cmd = cmd.arg(dest.as_ref());
    }

    cmd.spawn()?.wait()?;
    Ok(())
}

pub fn sparse_checkout_init_cone<P: AsRef<Path>>(cwd: P) -> Result<()> {
    Command::new("git")
        .current_dir(cwd)
        .arg("sparse-checkout")
        .arg("init")
        .arg("--cone")
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn sparse_checkout_set_path<P: AsRef<Path>>(cwd: P, path: &str) -> Result<()> {
    Command::new("git")
        .current_dir(cwd)
        .arg("sparse-checkout")
        .arg(format!("set {path}"))
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn pull_origin_branch<P: AsRef<Path>>(cwd: P, branch: Option<&str>) -> Result<()> {
    let branch = branch.unwrap_or("main");
    Command::new("git")
        .current_dir(cwd)
        .arg("pull")
        .arg("origin")
        .arg(branch)
        .spawn()?
        .wait()?;

    Ok(())
}
