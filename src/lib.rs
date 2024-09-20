pub(crate) mod error;
mod parser;

use http::{HeaderMap, Method, Uri};

pub use error::Error;

#[deprecated = "This type is deprecated. Use `Request` instead"]
#[derive(Debug, Clone, Default)]
pub struct ParsedRequest {
    pub method: Method,
    pub url: Uri,
    pub headers: HeaderMap,
    pub body: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Request(pub(crate) http::request::Request<bytes::Bytes>);

impl From<http::Request<bytes::Bytes>> for Request {
    fn from(req: http::Request<bytes::Bytes>) -> Self {
        Request(req)
    }
}

impl From<Request> for http::Request<bytes::Bytes> {
    fn from(req: Request) -> Self {
        req.0
    }
}
