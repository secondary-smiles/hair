use super::super::cli_lib::Arg;
use super::super::fn_lib::toggleenv_var;

pub const ARG: Arg = Arg {
    name: "Headers",
    short: Some('s'),
    long: Some("headers"),
    help: "Print only the headers of the http/s response",
};

pub fn run() {
    let print_headers = toggleenv_var("HAIR_PRINT_HEADERS");

    std::env::set_var("HAIR_PRINT_HEADERS", print_headers);
}
