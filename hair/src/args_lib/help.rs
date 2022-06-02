use super::{list_commands};

pub const NAME: &'static str = "Help";
pub const SHORT: Option<char> = Some('h');
pub const LONG: Option<&'static str> = Some("help");
pub const HELP: &'static str = "Print the help message";

pub fn run() {
    println!("Usage:\n\thair [OPTIONS] [method] URL\n");
    println!("Method:\n\t[method] is the optional HTTP method to use.\n\tIf not specified, the default method is GET.\n");
    println!("Options:\n");
    for command in list_commands() {
        let arg_short = match command.short {
            Some(s) => format!("-{}", s),
            None => "".to_string(),
        };

        let arg_long = match command.long {
            Some(l) => format!("--{}", l),
            None => "".to_string(),
        };
        let info_line;
        if arg_short != "" && arg_long != "" {
            info_line = format!("\t{}\t\t{}, {}\t\t{}", command.name, arg_short, arg_long, command.help);
        } else {
            info_line = format!("\t{}\t\t{}{}\t\t{}", command.name, arg_short, arg_long, command.help);
        }
        println!("{}", info_line);
    }
    std::process::exit(0);
}
