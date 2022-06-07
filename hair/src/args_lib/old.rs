pub const NAME: &'static str = "Old HTTP Method";
pub const SHORT: Option<char> = Some('o');
pub const LONG: Option<&'static str> = Some("old-http-method");
pub const HELP: &'static str = "Use HTTP 1.0 instead of 1.1";

use super::super::fn_lib::{toggleenv_var};

pub fn run() {
    let use_old_protocol = toggleenv_var("HAIR_USE_OLD_PROTOCOL");

    std::env::set_var("HAIR_USE_OLD_PROTOCOL", use_old_protocol);
}
