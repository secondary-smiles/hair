use super::super::cli_lib::{Arg, Run};
use super::list_commands;

pub struct Help {
    pub arg: Arg,
}

impl Run for Help {
    fn run(&self) {
        for arg in list_commands() {
            let short = match arg.short {
                Some(short) => format!("-{}", short),
                None => "".to_string(),
            };

            let long = match arg.long {
                Some(long) => format!("--{}", long),
                None => "".to_string(),
            };
            let info_line;
            if short != "" && long != "" {
                info_line = format!("{}\t\t{}, {}\t{}", arg.name, short, long, arg.help);
            } else {
                info_line = format!("{}\t\t{}{}\t\t{}", arg.name, short, long, arg.help);
            }
            println!("{}", info_line);
        }
        std::process::exit(0);
    }
}

pub static COMMAND: Help = Help {
    arg: Arg {
        name: "Help",
        short: Some('h'),
        long: Some("help"),
        help: "Print the help message",
    },
};
