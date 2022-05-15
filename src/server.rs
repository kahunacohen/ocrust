use crate::endpoint::Endpoint;
use crate::functions;
use serde::Deserialize;
use std::collections::HashMap;
use ureq;

pub struct Server<'a, P> {
    pub base_url: String,
    pub endpoints: Vec<Endpoint<'a, P>>,
}

impl<P> Server<'_, P> {
    pub fn new(base_url: String, endpoints: Vec<Endpoint<P>>) -> Server<P> {
        Server {
            base_url: functions::normalize_url(base_url),
            endpoints,
        }
    }
    fn request(&self, method: &str, uri: String) -> Result<ureq::Response, ureq::Error> {
        print!("{:?}\n", uri);
        ureq::request(&method, "https://www.google.com").call()
    }
    // pub fn get(&self, uri: String) -> String {
    //     self.request(Method::GET, uri)
    // }
}

#[cfg(test)]
mod test {
    use ureq::serde_json;

    use super::*;

    #[test]
    fn new_server_normalizes_base_url() {
        let server = Server::new(
            "https://papi.dev.ocdvlp.com/opportunities/".to_string(),
            vec![Endpoint::new(
                "/opportunities",
                "A test endpoint",
                HashMap::from([("GET", Some("x".to_string()))]),
            )],
        );
        assert_eq!(server.base_url, "https://papi.dev.ocdvlp.com/opportunities");
    }
    #[test]

    fn get() {
        let server = Server::new(
            "https://papi.dev.ocdvlp.com/opportunities/".to_string(),
            vec![Endpoint::new(
                "/opportunities",
                "A test endpoint",
                HashMap::from([("GET", Some("x".to_string()))]),
            )],
        );
        let resp = server
            .request("GET", String::from("/opportunities"))
            .unwrap();
        assert_eq!(resp.status(), 200);
        println!("{:?}", resp.get_url());
    }
}
