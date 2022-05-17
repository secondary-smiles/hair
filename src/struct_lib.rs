pub struct Url {
    pub host: String,
    pub path: String,
}

pub struct Response {
    pub headers: String,
    pub body: String,
}

pub struct Request {
    pub method: String,
    pub url: Url,
}
