use super::super::cli_lib::Arg;
use super::super::fn_lib::toggleenv_var;

pub const ARG: Arg = Arg {
    name: "Old HTTP Method",
    short: Some('o'),
    long: Some("old-http-method"),
    help: "Use HTTP 1.0 instead of 1.1",
};

pub fn run() {
    let use_old_protocol = toggleenv_var("HAIR_USE_OLD_PROTOCOL");

    std::env::set_var("HAIR_USE_OLD_PROTOCOL", use_old_protocol);
}
