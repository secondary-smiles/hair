pub fn error(message: &String, code: i32) {
    println!("ERROR: {}", message);
    std::process::exit(code);
}
