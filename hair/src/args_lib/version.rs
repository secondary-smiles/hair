use super::super::cli_lib::{VERSION};

pub const NAME: &'static str = "Version";
pub const SHORT: Option<char> = None;
pub const LONG: Option<&'static str> = Some("version");
pub const HELP: &'static str = "Print the current program version";

pub fn run() {
    println!("Hair -- version {}", VERSION);
    std::process::exit(0);
}
