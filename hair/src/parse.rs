use super::args_lib::{list_commands, run_command};
use super::struct_lib::{Request, Url};

pub fn parse_args(args: Vec<String>) -> Result<Request, &'static str> {
    let mut url: Url = Url {
        host: String::new(),
        path: String::new(),
    };
    let mut method: Option<String> = None;
    for arg in args {
        if arg.starts_with("-") {
            let mut has_run: bool = false;
            for command in list_commands() {
                let short = match command.short {
                    Some(s) => format!("-{}", s),
                    None => continue,
                };
                let long = match command.long {
                    Some(l) => format!("--{}", l),
                    None => continue,
                };
                if arg == short || arg == long {
                    run_command(&command.name);
                    has_run = true;
                }
            }
            if has_run == false {
                return Err("Invalid command");
            }
        } else if arg.starts_with("http") || arg.contains(".") && !arg.contains(" ") {
            url = parse_url(&arg).unwrap();
        } else if arg.to_uppercase() == arg {
            method = Some(arg.to_string());
        }
    }

    Ok(Request {
        method: method,
        url: url,
    })
}

fn parse_url(url: &String) -> Result<Url, String> {
    let mut host;
    let path;

    if url.split("//").collect::<Vec<&str>>().len() > 1 {
        host = url.split("//").collect::<Vec<&str>>()[1].to_string();
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

    Ok(Url {
        host: host,
        path: path,
    })
}
