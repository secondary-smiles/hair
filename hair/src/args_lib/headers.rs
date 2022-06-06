pub const NAME: &'static str = "Headers";
pub const SHORT: Option<char> = Some('s');
pub const LONG: Option<&'static str> = Some("headers");
pub const HELP: &'static str = "Print only the headers of the http/s response";

use super::super::fn_lib::{toggleenv_var};

pub fn run() {
    let print_headers = toggleenv_var("HAIR_PRINT_HEADERS");

    std::env::set_var("HAIR_PRINT_HEADERS", print_headers);
}
