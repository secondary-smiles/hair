pub mod help;
pub mod version;
pub mod verbose;

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
    ];
    commands
}

pub fn run_command(arg: &str) {
    match arg {
        "Help" => help::run(),
        "Version" => version::run(),
        "Verbose" => verbose::run(),
        _ => help::run(),
    }
}
