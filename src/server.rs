use std::collections::HashMap;

use crate::endpoint::{Endpoint, Method};
use crate::functions;

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
    pub fn get(&self, uri: String) -> String {
        "foo".to_string()
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
