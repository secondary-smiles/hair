pub const NAME: &'static str = "Body";
pub const SHORT: Option<char> = Some('b');
pub const LONG: Option<&'static str> = Some("body");
pub const HELP: &'static str = "Print only the body of the http/s response";

use super::super::fn_lib::{toggleenv_var};

pub fn run() {
    let print_body = toggleenv_var("HAIR_PRINT_BODY");

    std::env::set_var("HAIR_PRINT_BODY", print_body);
}
