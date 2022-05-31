use super::super::cli_lib::{Arg, Run, VERSION};

pub struct Version {
    pub arg: Arg,
}

impl Run for Version {
    fn run(&self) {
        println!("Hair -- version {}", VERSION);
        std::process::exit(0);
    }
}

pub static COMMAND: Version = Version {
    arg: Arg {
        name: "Version",
        short: Some('v'),
        long: Some("version"), 
        help: "Print the current program version",
    },
};
