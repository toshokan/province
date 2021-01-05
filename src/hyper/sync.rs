use crate::{Body, Communication, Execute};

pub struct Executor {
    client: hyper::Client<hyper::client::HttpConnector, hyper::Body>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            client: hyper::client::Client::new()
        }
    }
}

impl Execute for Executor {
    fn execute<C: Communication>(&self, t: C::Request) -> Result<C::Response, C::Error> {
	let req = C::into_request(t);
	unimplemented!()
    }
}
