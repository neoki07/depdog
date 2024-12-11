use std::path::Path;
use std::process::Command;

pub fn check_updates(directory: &Path) -> Result<String, std::io::Error> {
    let output = Command::new("npx")
        .args(&["npm-check-updates"])
        .current_dir(directory)
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
