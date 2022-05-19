use super::struct_lib::{Request, Url};

pub fn parse_args(args: Vec<String>) -> Result<Request, &'static str> {
    let mut url: Url = Url {
        host: String::new(),
        path: String::new(),
    };
    let mut method: Option<String> = None;
    for arg in args {
        if arg.starts_with("-") {
            // TODO: Parse `-` args
            return Err("Sorry, `-` args are not yet supported.");
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
