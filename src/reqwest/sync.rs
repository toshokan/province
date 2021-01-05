use crate::{Body, Communication, Execute};

pub struct Executor {
    client: reqwest::blocking::Client
}

impl Executor {
    pub fn new() -> Self {
	Self {
	    client: reqwest::blocking::Client::new()
	}
    }
}

impl From<Body> for reqwest::blocking::Body {
    fn from(_: Body) -> Self {
	reqwest::blocking::Body::from(String::new())
    }
}

impl Execute for Executor {
    fn execute<C: Communication>(&self, t: C::Request) -> Result<C::Response, C::Error> {
	use std::convert::TryInto;
	use reqwest::blocking::Request as RRequest;
	
	let req = C::into_request(t);
	let req: RRequest = req.try_into().unwrap_or_else(|_| panic!("What"));
	
	let rresp = self.client.execute(req).unwrap();
	let mut resp = http::Response::builder()
	    .status(rresp.status())
	    .version(rresp.version());
	for (h, v) in rresp.headers() {
	    resp = resp.header(h, v);
	}
	let body = Body::Bytes(rresp.bytes().unwrap());
	let resp = resp.body(body).unwrap();
	let value = C::from_response(resp);
	value
    }
}
