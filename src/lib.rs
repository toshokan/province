pub mod sync {
    #[cfg(feature = "isahc")]
    pub mod isahc {
	use crate::{Body, Communication, Execute};
	
	pub struct Executor {
	}

	impl From<Body> for isahc::Body {
	    fn from(_: Body) -> Self {
		isahc::Body::empty()
	    }
	}

	impl From<isahc::Body> for Body {
	    fn from(mut body: isahc::Body) -> Self {
		eprintln!("{:?}", body.len());
		if body.is_empty() {
		    Body::Empty
		} else {
		    use std::io::Read;
		    let mut buf = Vec::with_capacity(body.len().unwrap_or_default() as usize);
		    body.read_to_end(&mut buf).unwrap();
		    Body::Bytes(bytes::Bytes::from(buf))
		}
	    }
	}

	impl Execute for Executor {
	    fn execute<C: Communication>(&self, t: C::Request) -> Result<C::Response, C::Error> {
		use isahc::prelude::*;
		
		let req = C::into_request(t);
		let resp = req.send().unwrap();
		let (parts, body) = resp.into_parts();
		let body: Body = body.into();
		let resp = http::Response::from_parts(parts, body);
		C::from_response(resp)
	    }
	}
    }

    #[cfg(feature = "reqwest")]
    pub mod reqwest {
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
    }
}

#[derive(Debug)]
enum Body {
    Empty,
    Bytes(bytes::Bytes)
}

impl Body {
    fn as_slice(&self) -> &[u8] {
	match self {
	    Self::Empty => &[],
	    Self::Bytes(b) => b.as_ref()
	}
    }
}

trait Communication {
    type Request;
    type Response;
    type Error;

    fn into_request(r: Self::Request) -> http::request::Request<Body>;
    fn from_response(r: http::Response<Body>) -> Result<Self::Response, Self::Error>;
}

trait Execute {
    fn execute<C: Communication>(&self, t: C::Request) -> Result<C::Response, C::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct IpReq;
    #[derive(Debug)]
    pub struct IpResp(String);

    impl Communication for IpReq {
	type Request = Self;
	type Response = IpResp;
	type Error = ();

	fn into_request(_: Self::Request) -> http::Request<Body> {
	    use http::Request;
	    
	    Request::get("http://whatismyip.akamai.com")
		.body(Body::Empty)
		.unwrap()
	}

	fn from_response(r: http::Response<Body>) -> Result<Self::Response, Self::Error> {
	    let bytes = r.into_body();
	    let content = String::from_utf8(bytes.as_slice().to_vec()).unwrap();
	    Ok(IpResp(content))
	}
    }

    fn ip_test_execute(e: impl Execute) -> Result<(), ()> {
	let resp: IpResp = e.execute::<IpReq>(IpReq)?;
	dbg!(&resp);
	let ip_parts: Vec<_> = resp.0.split(".").collect();
	assert_eq!(ip_parts.len(), 4);
	Ok(())
    }
    
    #[test]
    #[cfg(feature = "reqwest")]
    fn reqwest() {
	use crate::sync::reqwest::Executor;
	let exec = Executor::new();
	ip_test_execute(exec).unwrap()
    }

    #[test]
    #[cfg(feature = "isahc")]
    fn isahc() {
	use crate::sync::isahc::Executor;
	let exec = Executor{};
	ip_test_execute(exec).unwrap()
    }
}
