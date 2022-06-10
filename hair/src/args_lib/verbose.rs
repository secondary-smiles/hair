use super::super::cli_lib::Arg;
use super::super::fn_lib::toggleenv_var;

pub const ARG: Arg = Arg {
    name: "Verbose",
    short: Some('v'),
    long: Some("verbose"),
    help: "Print the entire interaction with the server",
};

pub fn run() {
    let print_verbose = toggleenv_var("HAIR_PRINT_VERBOSE");

    std::env::set_var("HAIR_PRINT_VERBOSE", print_verbose);
}
