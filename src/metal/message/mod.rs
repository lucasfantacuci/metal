use super::http::Method;
use super::http::Status;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: Path,
    pub headers: Headers,
    pub cookies: Cookies
}

#[derive(Debug)]
pub struct Response {
    pub status : Status,
    pub headers: Headers
}

impl Default for Response {
    fn default() -> Response {
        Response {
            status : Status::Ok,
            headers : Headers::default()
        }
    }
}

#[derive(Debug)]
pub struct Path {
    pub path: String
}

#[derive(Debug)]
pub struct Headers {
    headers : Vec<Header>
}

impl Headers {
    pub fn add_header(&mut self, header: Header) {
        &self.headers.push(header);
    }
}

impl Default for Headers {
    fn default() -> Headers {
        Headers {
            headers : Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub value: String
}

#[derive(Debug)]
pub struct Cookies {
    cookies : Vec<Cookie>
}

impl Cookies {
    pub fn add_cookie(&mut self, cookie: Cookie) {
        &self.cookies.push(cookie);
    }
}

impl Default for Cookies {
    fn default() -> Cookies {
        Cookies {
            cookies: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Cookie {
    pub name: String,
    pub value: String
}