use super::args_lib::{list_commands, run_command};
use super::struct_lib::{Request, Url};

pub fn parse_args(args: Vec<String>) -> Result<Request, String> {
    let mut url: Url = Url {
        host: String::new(),
        path: String::new(),
        port: None,
    };
    let mut method: Option<String> = None;
    if args.len() < 2 {
        eprintln!("Error: No arguments provided\n");
        run_command("Help");
    }
    let mut url_provided = false;
    for arg in args {
        if arg.starts_with("-") {
            let mut has_run: bool = false;
            for command in list_commands() {
                let short = match command.short {
                    Some(s) => format!("-{}", s),
                    None => "".to_string(),
                };
                let long = match command.long {
                    Some(l) => format!("--{}", l),
                    None => "".to_string(),
                };
                if arg == short || arg == long {
                    run_command(&command.name);
                    has_run = true;
                }
            }
            if has_run == false {
                match parse_single_args(&arg, &mut has_run) {
                    Ok(()) => (),
                    Err(e) => return Err(e),
                }
            }
            if has_run == false {
                return Err(format!("Invalid command: {:?}", arg));
            }
        } else if arg.contains("://")
            || arg.contains(".")
            || arg.contains(":") && !arg.contains(" ")
        {
            url = parse_url(&arg).unwrap();
            url_provided = true;
        } else if arg.to_uppercase() == arg {
            method = Some(arg.to_string());
        }
    }
    if url_provided == false {
        return Err("No URL provided".to_string());
    }

    Ok(Request {
        method: method,
        url: url,
    })
}

fn parse_url(url: &String) -> Result<Url, String> {
    let mut host;
    let path;
    let port: Option<String>;

    if url.split("://").collect::<Vec<&str>>().len() > 1 {
        host = url.split("://").collect::<Vec<&str>>()[1].to_string();
    } else {
        host = url.to_string();
    }

    if host.split("/").collect::<Vec<&str>>().len() > 1 {
        path = format!(
            "/{}",
            host.split("/")
                .collect::<Vec<&str>>()
                .drain(1..)
                .collect::<Vec<&str>>()
                .join("/")
        );
        host = host.split("/").collect::<Vec<&str>>()[0].to_string();
    } else {
        path = "/".to_string();
    }

    if host.split(":").collect::<Vec<&str>>().len() > 1 {
        let port_opt = host.split(":").collect::<Vec<&str>>()[1]
            .to_string()
            .split("/")
            .collect::<Vec<&str>>()[0]
            .to_string();
        if port_opt.parse::<u16>().is_ok() {
            port = Some(port_opt);
        } else {
            return Err(format!("Invalid port: {}", port_opt));
        }
    } else {
        port = None;
    }

    if port != None {
        host = host.split(":").collect::<Vec<&str>>()[0].to_string();
    }

    Ok(Url {
        host: host,
        path: path,
        port: port,
    })
}

fn parse_single_args(arg: &str, has_run: &mut bool) -> Result<(), String> {
    let mut error_args: Vec<String> = vec![];
    for a in arg.chars() {
        if a.is_alphanumeric() {
            *has_run = false;
            for command in list_commands() {
                let short = match command.short {
                    Some(s) => s,
                    None => ' ',
                };
                if short == a {
                    run_command(&command.name);
                    *has_run = true;
                }
            }
            if has_run == &false {
                let err_arg = format!("\"-{}\"", a);
                error_args.push(err_arg);
            }
        }
    }
    if error_args.len() > 0 && error_args.len() < 2 {
        return Err(format!("Invalid command: {}", error_args.join(" ")));
    } else if error_args.len() > 1 {
        return Err(format!("Invalid commands: {}", error_args.join(" ")));
    }
    Ok(())
}
