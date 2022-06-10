use super::super::cli_lib::Arg;

pub const ARG: Arg = Arg {
    name: "Body",
    short: Some('b'),
    long: Some("body"),
    help: "Print only the body of the http/s response",
};

use super::super::fn_lib::toggleenv_var;

pub fn run() {
    let print_body = toggleenv_var("HAIR_PRINT_BODY");

    std::env::set_var("HAIR_PRINT_BODY", print_body);
}
