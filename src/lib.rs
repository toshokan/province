#[cfg(feature = "client-isahc")]
pub mod isahc;

#[cfg(feature = "client-reqwest")]
pub mod reqwest;

#[derive(Debug)]
pub enum Body {
    Empty,
    Bytes(bytes::Bytes),
}

impl Body {
    fn as_slice(&self) -> &[u8] {
        match self {
            Self::Empty => &[],
            Self::Bytes(b) => b.as_ref(),
        }
    }
}

pub trait Communication {
    type Request;
    type Response;
    type Error;

    fn into_request(r: Self::Request) -> http::request::Request<Body>;
    fn from_response(r: http::Response<Body>) -> Result<Self::Response, Self::Error>;
}

pub trait Execute {
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

    pub fn ip_test_execute(e: impl Execute) -> Result<(), ()> {
        let resp: IpResp = e.execute::<IpReq>(IpReq)?;
        dbg!(&resp);
        let ip_parts: Vec<_> = resp.0.split(".").collect();
        assert_eq!(ip_parts.len(), 4);
        Ok(())
    }
}
