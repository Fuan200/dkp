use std::{
    io,
    process::{Command, Stdio},
};

pub fn initialize(project_path: &str) -> io::Result<()> {
    let status = Command::new("git")
        .arg("init")
        .current_dir(project_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other("git init failed"))
    }
}
