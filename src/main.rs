use clap::Command;

fn main() {
    Command::new("depdog")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A tool to check the versions of project dependencies")
        .get_matches();

    println!("Hello, world!");
}
