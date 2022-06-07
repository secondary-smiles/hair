pub fn error(message: &String, code: i32) {
    eprintln!("ERROR: {}", message);
    std::process::exit(code);
}

pub fn fenv_var(name: &str) -> String {
    match std::env::var(name) {
        Ok(v) => v,
        Err(_) => {
            let msg = format!("Could not find ENV variable: {}", name);
            error(&msg, 1);
            "".to_string()
        },
    }
}

pub fn toggleenv_var(name: &str) -> String {
    match std::env::var(name) {
        Ok(v) => match v {
            _ if v == "0".to_string() => "1".to_string(),
            _ if v == "1".to_string() => "0".to_string(),
            _ => "0".to_string(),
        },
        Err(_) => {
            let msg = format!("Could not find ENV variable: {}", name);
            error(&msg, 1);
            "0".to_string()
        },
    }
}
