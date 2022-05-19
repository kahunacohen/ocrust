use crate::endpoint::Endpoint;
use crate::functions;
use serde::Deserialize;
use std::{collections::HashMap, fmt};
use ureq;
use httpmock::prelude::*;

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
    no_endpoint: bool,
    status_code: Option<u16>,
    status_text: Option<String>,
    url: String,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        return match &self.status_text {
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
        let url = format!("{}{}", self.base_url, uri);
        for endpoint in &self.endpoints {
            // Interate through server's endpoints and only make request if endpoint
            // exists.
            if uri == endpoint.uri {
                return match ureq::request(&method, &url).call() {
                    Ok(response) => Ok(response),
                    Err(ureq::Error::Status(code, response)) => Err(ResponseError {
                        no_endpoint: false,
                        status_code: Some(code),
                        status_text: Some(response.status_text().to_string()),
                        url: response.get_url().to_string(),
                    }),
                    _ => Err(ResponseError {
                        no_endpoint: false,
                        status_code: None,
                        status_text: None,
                        url,
                    }),
                };
            }
        }
        return Err(ResponseError {
            no_endpoint: true,
            status_code: None,
            status_text: None,
            url,
        });

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
    fn new_server_normalizes_base_url_leaves_slashes_alone() {
        let server = Server::new(
            "https://papi.dev.ocdvlp.com".to_string(),
            vec![Endpoint::new(
                "/opportunities",
                "A test endpoint",
                HashMap::from([("GET", Some("x".to_string()))]),
            )],
        );
        assert_eq!(server.base_url, "https://papi.dev.ocdvlp.com");
    }
    #[test]
    fn new_server_normalizes_base_url_removes_trailing_slash() {
        let server = Server::new(
            "https://papi.dev.ocdvlp.com/".to_string(),
            vec![Endpoint::new(
                "/opportunities",
                "A test endpoint",
                HashMap::from([("GET", Some("x".to_string()))]),
            )],
        );
        assert_eq!(server.base_url, "https://papi.dev.ocdvlp.com");
    }
    #[test]
    fn request_succeeds() {

        let mock_server = MockServer::start();

        // Create a mock on the server.
        let hello_mock = mock_server.mock(|when, then| {
            when.method(GET)
                .path("/opportunities/");
            then.status(200)
                .header("content-type", "application/json")
                .body("ohi");
        });
        println!("{}", mock_server.url("/opportunities/"));
        

        let server = Server::new(
            "https://papi.dev.ocdvlp.com".to_string(),
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
    #[test]
    fn request_fails_bad_status_code() {
        let server = Server::new(
            "https://papi.dev.ocdvlp.com".to_string(),
            vec![Endpoint::new(
                "/foo/",
                "A test endpoint",
                HashMap::from([("GET", Some("x".to_string()))]),
            )],
        );
        match server.request("GET", String::from("/foo/")) {
            Ok(_) => panic!("request should have failed"),
            Err(err) => {
                assert_eq!(err.no_endpoint, false);
                assert_eq!(err.status_code, Some(404));
                assert_eq!(err.status_text, Some("Not Found".to_string()))
            }
        }
    }
}
