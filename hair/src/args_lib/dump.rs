use super::super::cli_lib::Arg;
use super::super::fn_lib::toggleenv_var;

pub const ARG: Arg = Arg {
    name: "Parse Error Dump",
    short: Some('d'),
    long: Some("parse-err-dump"),
    help: "Dump recieved data to stdout on a parse error",
};

pub fn run() {
    let parse_err_dump = toggleenv_var("HAIR_PARSE_ERR_DUMP");

    std::env::set_var("HAIR_PARSE_ERR_DUMP", parse_err_dump);
}
