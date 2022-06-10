use super::super::cli_lib::{Arg, VERSION};

pub const ARG: Arg = Arg {
    name: "Version",
    short: None,
    long: Some("version"),
    help: "Print the current program version",
};

pub fn run() {
    println!("Hair -- version {}", VERSION);
    std::process::exit(0);
}
