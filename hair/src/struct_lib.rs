#[derive(Clone)]
pub struct Url {
    pub host: String,
    pub path: String,
    pub port: Option<String>,
}

pub struct Response {
    pub headers: String,
    pub body: String,
}

pub struct Request {
    pub method: Option<String>,
    pub url: Url,
}

impl std::fmt::Debug for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Url {{ host: {}, path: {} }}", self.host, self.path)
    }
}

impl std::fmt::Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Request {{ method: {:?}, url: {:?} }}",
            self.method, self.url
        )
    }
}
