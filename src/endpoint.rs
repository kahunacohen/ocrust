use std::collections::HashMap;

use regex::Regex;

use crate::functions;

/// A "generic" payload we can use as a type parameter
/// to Endpoint when instantiating. Otherwise the compiler is not
/// happy. For example, when we instantiate an endpoint and assign it
/// to a variable, the type checker isn't happy unless we provide explicit
/// annotation on the variable:
/// ```
/// let e: Endpoint<Payload> = Endpoint::new(
///     String::from("/foo"),
///     HashMap::from([("GET", None)]),
/// );
/// ```
pub struct Payload {
    pub data: Option<String>,
}

/// Represents an API endpoint. The
/// generic parameter `P` stands for some kind
/// of payload. So the methods field is a vector
/// of hashmaps that map HTTP methods, "GET" etc. to
/// a payload.
pub struct Endpoint<P> {
    pub description: String,
    pub methods: HashMap<String, Option<P>>,
    pub uri: String,
    url: String,
}
impl<P> Endpoint<P> {
    /// Creates a new endpoint. E.g.:
    /// ```
    /// // An endpoint representing the URI `/foo`
    /// //implementing `GET` with no payload.
    ///  let e: Endpoint<Payload> = Endpoint::new(
    ///    "/foo".to_string(),
    ///    "A test endpoint",
    ///    HashMap::from([("GET", None)])
    ///  );
    /// ```
    pub fn new(
        uri: String,
        description: String,
        methods: HashMap<String, Option<P>>,
    ) -> Endpoint<P> {
        let re = Regex::new(r"^/|/$").unwrap();
        Endpoint {
            description,
            methods,
            uri: format!("/{}/", re.replace_all(&uri, "")),
            url: "https://www.google.com".to_string(),
        }
    }
    /// Whether the endpoint implements the passed HTTP method.
    pub fn implements_method(&self, method: &str) -> bool {
        self.methods.contains_key(method)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn endpoint_handles_leading_slash() {
        let e: Endpoint<Payload> = Endpoint::new(
            "/foo".to_string(),
            "A test endpoint".to_string(),
            HashMap::from([("GET".to_string(), None)]),
        );
        assert_eq!(e.uri, "/foo/".to_string());
    }
    #[test]
    fn endpoint_handles_trailing_slash() {
        let e: Endpoint<Payload> = Endpoint::new(
            "/foo".to_string(),
            "A test endpoint".to_string(),
            HashMap::from([("GET".to_string(), None)]),
        );
        assert_eq!(e.uri, "/foo/");
    }
    #[test]
    fn endpoint_handles_existing_leading_trailing_slash() {
        let e: Endpoint<Payload> = Endpoint::new(
            "/foo/".to_string(),
            "A test endpoint".to_string(),
            HashMap::from([("GET".to_string(), None)]),
        );
        assert_eq!(e.uri, "/foo/".to_string());
    }
    #[test]
    fn endpoint_handles_no_slashes() {
        let e: Endpoint<Payload> = Endpoint::new(
            "foo".to_string(),
            "A test endpoint".to_string(),
            HashMap::from([("GET".to_string(), None)]),
        );
        assert_eq!(e.uri, "/foo/".to_string());
    }
    #[test]
    fn implements_method() {
        let e: Endpoint<Payload> = Endpoint::new(
            "foo".to_string(),
            "A test endpoint".to_string(),
            HashMap::from([("GET".to_string(), None)]),
        );
        assert_eq!(e.implements_method("GET"), true);
    }
    #[test]
    fn doesnt_implement_method() {
        let e: Endpoint<Payload> = Endpoint::new(
            "foo".to_string(),
            "A test endpoint".to_string(),
            HashMap::from([("GET".to_string(), None)]),
        );
        assert_eq!(e.implements_method("POST"), false);
    }
}
