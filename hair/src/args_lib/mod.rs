mod help;
mod version;
mod verbose;
mod body;
mod headers;
mod old;

use super::cli_lib::{Arg};

pub fn list_commands() -> Vec<Arg> {
    let commands: Vec<Arg> = vec![
        Arg {
            name: help:: NAME,
            short: help::SHORT,
            long: help:: LONG,
            help: help:: HELP,
        },
        Arg {
            name: version:: NAME,
            short: version::SHORT,
            long: version:: LONG,
            help: version:: HELP,
        },
        Arg {
            name: verbose:: NAME,
            short: verbose::SHORT,
            long: verbose:: LONG,
            help: verbose:: HELP,
        },
        Arg {
            name: body:: NAME,
            short: body::SHORT,
            long: body:: LONG,
            help: body:: HELP,
        },
        Arg {
            name: headers:: NAME,
            short: headers::SHORT,
            long: headers:: LONG,
            help: headers:: HELP,
        },
        Arg {
            name: old:: NAME,
            short: old::SHORT,
            long: old:: LONG,
            help: old:: HELP,
        },
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
        _ => help::run(),
    }
}
