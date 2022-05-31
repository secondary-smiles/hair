use super::super::cli_lib::{ Arg, Run };

pub struct Version {
    pub arg: Arg,
    pub can_run: bool,
}

impl Run for Version {
    fn run(&self) {
        println!("Hair -- version 0.1.3");
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
    can_run: true,
};
