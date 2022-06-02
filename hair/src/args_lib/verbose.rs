pub const NAME: &'static str = "Verbose";
pub const SHORT: Option<char> = Some('v');
pub const LONG: Option<&'static str> = Some("verbose");
pub const HELP: &'static str = "Print the entire interaction with the server";

pub fn run() {
    let mut print_verbose = std::env::var("HAIR_PRINT_VERBOSE")
        .unwrap()
        .parse::<i32>()
        .unwrap();
    print_verbose = match print_verbose {
        0 => 1,
        1 => 0,
        _ => 0,
    };
    std::env::set_var("HAIR_PRINT_VERBOSE", print_verbose.to_string());
}
