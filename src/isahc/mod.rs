#[cfg(feature = "sync")]
pub mod sync;

use crate::Body;

impl From<Body> for isahc::Body {
    fn from(_: Body) -> Self {
        isahc::Body::empty()
    }
}

impl From<isahc::Body> for Body {
    fn from(mut body: isahc::Body) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        use sync::Executor;
        let exec = Executor{};
        crate::tests::ip_test_execute(exec).unwrap()
    }
}
