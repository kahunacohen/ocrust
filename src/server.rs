use crate::endpoint::{Endpoint, Method};
use crate::functions;
use std::collections::HashMap;
use ureq;

pub struct Server<P> {
    pub base_url: String,
    pub endpoints: Vec<Endpoint<P>>,
}

impl<P> Server<P> {
    pub fn new(base_url: String, endpoints: Vec<Endpoint<P>>) -> Server<P> {
        Server {
            base_url: functions::normalize_url(base_url),
            endpoints,
        }
    }
    fn request(&self, method: Method, uri: String) ->ureq::Response {
        ureq::request("GET", "https://www.google.com").call()
    }
    /// Make an HTTP `GET` request to the given `uri`.
    pub fn get(&self, uri: String) -> String {
        self.request(Method::GET, uri)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_server_normalizes_base_url() {
        let server = Server::new(
            "https://papi.dev.ocdvlp.com/opportunities/".to_string(),
            vec![Endpoint::new(
                "/opportunities".to_string(),
                "A test endpoint".to_string(),
                HashMap::from([(Method::GET, Some("x".to_string()))]),
            )],
        );
        assert_eq!(server.base_url, "https://papi.dev.ocdvlp.com/opportunities");
    }
}
