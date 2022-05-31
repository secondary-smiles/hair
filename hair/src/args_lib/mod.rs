pub mod help;
pub mod version;

use super::cli_lib::{Arg, Run};

pub fn list_commands() -> Vec<Arg> {
    vec![
        help::COMMAND.arg,
        version::COMMAND.arg,
    ]
}

pub fn run_command(arg: &str) {
    match arg {
        "Help" => help::COMMAND.run(),
        "Version" => version::COMMAND.run(),
        _ => println!("No command found"),
    }
}
