mod body;
mod dump;
mod headers;
mod help;
mod old;
mod verbose;
mod version;

use super::cli_lib::Arg;

pub fn list_commands() -> Vec<Arg> {
    let commands: Vec<Arg> = vec![
        help::ARG,
        version::ARG,
        verbose::ARG,
        body::ARG,
        headers::ARG,
        old::ARG,
        dump::ARG,
    ];
    commands
}

pub fn run_command(arg: &str) {
    match arg {
        "Help" => help::run(),
        "Version" => version::run(),
        "Verbose" => verbose::run(),
        "Body" => body::run(),
        "Headers" => headers::run(),
        "Old HTTP Method" => old::run(),
        "Parse Error Dump" => dump::run(),
        _ => help::run(),
    }
}
