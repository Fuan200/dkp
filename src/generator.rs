use crate::prompts::Platform;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn create_project(name: &str, platform: &Platform) -> io::Result<()> {
    let project_path = Path::new(name);
    let template_path = Path::new("templates").join(platform.template_directory());

    fs::create_dir(project_path)?;
    copy_directory(&template_path, project_path)
}

fn copy_directory(source: &Path, destination: &Path) -> io::Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path: PathBuf = destination.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            fs::create_dir(&destination_path)?;
            copy_directory(&source_path, &destination_path)?;
        } else {
            fs::copy(&&source_path, &destination_path)?;
        }
    }
    Ok(())
}
