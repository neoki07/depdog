use std::path::Path;

use clap::Command;

mod npm_checker;
mod project_detector;

fn main() {
    Command::new("depdog")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A tool to check the versions of project dependencies")
        .get_matches();

    let current_dir = Path::new(".");

    if project_detector::is_npm_project(current_dir) {
        match npm_checker::check_updates(current_dir) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        println!("This is not an npm project.");
    }
}
