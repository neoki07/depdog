use std::path::Path;

pub fn is_npm_project(directory: &Path) -> bool {
    directory.join("package.json").exists()
}

pub fn is_cargo_project(directory: &Path) -> bool {
    directory.join("Cargo.toml").exists()
}
