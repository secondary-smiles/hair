pub const NAME: &'static str = "Verbose";
pub const SHORT: Option<char> = Some('v');
pub const LONG: Option<&'static str> = Some("verbose");
pub const HELP: &'static str = "Print the entire interaction with the server";

use super::super::fn_lib::{error};

pub fn run() {
    let print_verbose = match std::env::var("HAIR_PRINT_VERBOSE") {
        Ok(v) => match v {
            _ if v == "0".to_string() => "1".to_string(),
            _ if v == "1".to_string() => "0".to_string(),
            _ => "0".to_string(),
        },
        Err(_) => {
            let msg = "Could not find ENV variable".to_string();
            error(&msg, 1);
            "0".to_string()
        },
    };

    std::env::set_var("HAIR_PRINT_VERBOSE", print_verbose);
}
