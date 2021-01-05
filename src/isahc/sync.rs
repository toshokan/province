use crate::{Body, Communication, Execute};

pub struct Executor;

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
