pub fn error(message: &str, code: i32) {
    println!("ERROR: {}", message);
    std::process::exit(code);
}
