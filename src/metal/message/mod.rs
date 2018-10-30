use super::http::Method;

pub struct Request {
    pub method: Method,
    pub path: Path
}

pub struct Response;

#[derive(Debug)]
pub struct Path {
    pub path: String
}
