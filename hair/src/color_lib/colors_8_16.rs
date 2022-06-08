pub fn c(color: &str, fg: bool) -> &'static str {
    if fg {
        match color {
            "black" => return "\x1b[30m",
            "red" => return "\x1b[31m",
            "green" => return "\x1b[32m",
            "yellow" => return "\x1b[33m",
            "blue" => return "\x1b[34m",
            "magenta" => return "\x1b[35m",
            "cyan" => return "\x1b[36m",
            "white" => return "\x1b[37m",
            _ => return "",
        }
    } else if !fg {
       match color {
            "black" => return "\x1b[40m",
            "red" => return "\x1b[41m",
            "green" => return "\x1b[42m",
            "yellow" => return "\x1b[43m",
            "blue" => return "\x1b[44m",
            "magenta" => return "\x1b[45m",
            "cyan" => return "\x1b[46m",
            "white" => return "\x1b[47m",
            _ => return "",
        }
    }
    return "";
}

pub fn r() -> &'static str {
    return "\x1b[0m";
}
