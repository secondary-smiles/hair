use super::lib::{Request, Url};

pub fn parse_args(args: &mut Vec<String>) -> Result<Request, String> {
    if args.len() != 3 {
        return Err("Error: Invalid number of arguments".to_string());
    }

    let mut host;
    let path;

    if args[2].split("//").collect::<Vec<&str>>().len() > 1 {
        host = args[2].split("//").collect::<Vec<&str>>()[1].to_string();
    } else {
        host = args[2].to_string();
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

    Ok(Request {
        method: args[1].to_string(),
        url: Url {
            host: host,
            path: path,
        },
    })
}
