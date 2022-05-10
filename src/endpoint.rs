use std::collections::HashMap;

use crate::functions;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
}

// A "base" payload
pub struct Payload {
    data: Option<String>
}
// Represents an API endpoint. The
// generic parameter `P` stands for some kind
// of payload. So the methods field is a vector
// of hashmaps that map HTTP methods, "GET" etc. to
// a payload.
pub struct Endpoint<P> {
    pub uri: String,
    pub methods: HashMap<Method, Option<P>>,
}
impl<P> Endpoint<P> {
    pub fn new(uri: String, methods: HashMap<Method, Option<P>>) -> Endpoint<P> {
        Endpoint {
            uri: functions::normalize_uri(&uri),
            methods,
        }
    }

    // pub fn implements_method(&self, method: String) -> bool {
    //     self.methods.contains(&method)
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn endpoint_handles_leading_slash() {
        let e: Endpoint<Payload> = Endpoint::new(
            String::from("/foo"),
            HashMap::from([(Method::GET, None)]),
        );
        assert_eq!(e.uri, "/foo/");
    }
    #[test]
    fn endpoint_handles_trailing_slash() {
        let e: Endpoint<Payload> = Endpoint::new(
            String::from("/foo"),
            HashMap::from([(Method::GET, None)]),
        );
        assert_eq!(e.uri, "/foo/");
    }
    #[test]
    fn endpoint_handles_existing_leading_trailing_slash() {
        let e: Endpoint<Payload> = Endpoint::new(
            String::from("/foo/"),
            HashMap::from([(Method::GET, None)]),
        );
        assert_eq!(e.uri, "/foo/");
    }
    #[test]
    fn endpoint_handles_no_slashes() {
        let e: Endpoint<Payload> = Endpoint::new(
            String::from("foo"),
            HashMap::from([(Method::GET, None)]),
        );
        assert_eq!(e.uri, "/foo/");
    }
    // #[test]
    // fn implements_method() {
    //     let e = Endpoint {
    //         uri: "/foo".to_string(),
    //         methods: vec!["GET".to_string()],
    //     };
    //     assert_eq!(e.implements_method("GET".to_string()), true);
    // }
    // #[test]
    // fn doesnt_implement_method() {
    //     let e = Endpoint {
    //         uri: "/foo".to_string(),
    //         methods: vec!["GET".to_string()],
    //     };
    //     assert_eq!(e.implements_method("POST".to_string()), false);
    // }
}
