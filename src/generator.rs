use std::{fs, io, path::Path};

pub fn create_project(name: &str) -> io::Result<()> {
    fs::create_dir(Path::new(name))
}
