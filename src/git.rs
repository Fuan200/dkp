use std::{io, process::Command};

pub fn initialize(project_path: &str) -> io::Result<()> {
    let status = Command::new("git")
        .arg("init")
        .current_dir(project_path)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other("git init failed"))
    }
}
