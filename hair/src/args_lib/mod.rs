pub mod help;
pub mod version;

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
    ];
    commands
}

/*
pub fn match_arg(arg: &str) -> Result<Arg, &'static str> {
    for command in list_commands() {
        let short = match command.short {
            Some(short) => format!("-{}", short),
            None => "".to_string(),
        };
        let long = match command.long {
            Some(long) => format!("--{}", long),
            None => "".to_string(),
        };
        if short == arg || long == arg {
            return Ok(command);
        }
    }
    Err("No matching command found")
}
*/

pub fn run_command(arg: &str) {
    match arg {
        "Help" => help::run(),
        "Version" => version::run(),
        _ => println!("No command found"),
    }
}
