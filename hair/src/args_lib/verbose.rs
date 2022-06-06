pub const NAME: &'static str = "Verbose";
pub const SHORT: Option<char> = Some('v');
pub const LONG: Option<&'static str> = Some("verbose");
pub const HELP: &'static str = "Print the entire interaction with the server";

use super::super::fn_lib::{toggleenv_var};

pub fn run() {
    let print_verbose = toggleenv_var("HAIR_PRINT_VERBOSE");

    std::env::set_var("HAIR_PRINT_VERBOSE", print_verbose);
}
