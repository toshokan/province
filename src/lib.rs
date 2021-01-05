#[cfg(feature = "isahc")]
pub mod isahc;

#[cfg(feature = "reqwest")]
pub mod reqwest;

#[derive(Debug)]
enum Body {
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
        use crate::reqwest::sync::Executor;
        let exec = Executor::new();
        ip_test_execute(exec).unwrap()
    }

    #[test]
    #[cfg(feature = "isahc")]
    fn isahc() {
        use crate::isahc::sync::Executor;
        let exec = Executor {};
        ip_test_execute(exec).unwrap()
    }
}
