use crate::endpoint::Endpoint;
use crate::functions;
use serde::Deserialize;
use std::io::Write;
use std::{collections::HashMap, fmt};
use ureq;
pub struct Server<'a, P> {
    pub base_url: String,
    pub endpoints: Vec<Endpoint<'a, P>>,
}

/// A custom response error that wraps ureq responses and
/// the possiblity that a url can be requested that is not a defined
/// endpoint on the server instance.
#[derive(Debug)]
pub struct ResponseError {
    // As opposed to a 404, this means the endpoint doesn't exist
    // in the server instance.
    endpoint_doesnt_exit: bool,
    status_code: Option<u16>,
    status_text: Option<String>,
    url: String,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let err_msg = match self.status_text {
            Some(txt) if txt != "OK" => write!(
                f,
                "The {} returned {}, status code: {}",
                self.url,
                self.status_code.unwrap(),
                txt
            ),
            Some(txt) => write!(
                f,
                "The url, {} is not a defined endpoint on the server",
                self.url
            ),
            _ => write!(f, "undefined error calling {}", self.url),
        };

        write!(f, "{}", err_msg)
    }
}

impl<P> Server<'_, P> {
    pub fn new(base_url: String, endpoints: Vec<Endpoint<P>>) -> Server<P> {
        Server {
            base_url: functions::normalize_url(base_url),
            endpoints,
        }
    }
    fn request(&self, method: &str, uri: String) -> Result<ureq::Response, ResponseError> {
        println!("{}", uri);
        for endpoint in &self.endpoints {
            println!("{}", endpoint.uri);
            if uri == endpoint.uri {
                let call_result = ureq::request(&method, "https://www.google.com").call();
                
            }
        }

        //return ureq::ErrorKind::InvalidUrl(format!("No endpoint found with URI: {}", uri))
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

    fn request() {
        let server = Server::new(
            "https://papi.dev.ocdvlp.com/opportunities/".to_string(),
            vec![Endpoint::new(
                "/opportunities/",
                "A test endpoint",
                HashMap::from([("GET", Some("x".to_string()))]),
            )],
        );
        let resp = server
            .request("GET", String::from("/opportunities/"))
            .unwrap();
        assert_eq!(resp.status(), 200);
        println!("{:?}", resp.get_url());
    }
}
