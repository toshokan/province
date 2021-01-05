use crate::{Body, Communication, Execute};

pub struct Executor {
    client: reqwest::blocking::Client,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
        }
    }
}

fn http_to_request(req: http::Request<Body>) -> reqwest::blocking::Request {
    use reqwest::blocking::Request;

    let (parts, body) = req.into_parts();
    let url = url::Url::parse(&parts.uri.to_string()).unwrap();
    let mut reqw = Request::new(parts.method, url);
    *reqw.headers_mut() = parts.headers;

    if let Body::Bytes(b) = body {
        *reqw.body_mut() = Some(reqwest::blocking::Body::from(b.to_vec()))
    }

    reqw
}

fn response_to_http(respq: reqwest::blocking::Response) -> http::Response<Body> {
    let version = respq.version();
    let status = respq.status();
    let headers = respq.headers().clone();
    let body = respq.bytes().unwrap();
    let body = if body.len() == 0 {
        Body::Empty
    } else {
        Body::Bytes(body)
    };

    let mut resp = http::Response::new(body);
    *resp.version_mut() = version;
    *resp.status_mut() = status;
    *resp.headers_mut() = headers;

    resp
}

impl Execute for Executor {
    fn execute<C: Communication>(&self, t: C::Request) -> Result<C::Response, C::Error> {
        let req = http_to_request(C::into_request(t));
        let respq = self.client.execute(req).unwrap();
        let resp = response_to_http(respq);

        C::from_response(resp)
    }
}
