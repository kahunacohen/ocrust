use crate::endpoint::Endpoint;
use crate::functions;
use serde::Deserialize;
use std::{collections::HashMap, fmt};
use ureq;

/// Server that collects endpoints, each with
/// a payload.
pub struct Server<P> {
    pub base_url: String,
    pub endpoints: Vec<Endpoint<P>>,
}

/// A custom response error that wraps ureq responses and
/// the possiblity that a url can be requested that is not a defined
/// endpoint on the server instance.
#[derive(Debug)]
pub struct ResponseError {
    // As opposed to a 404, this means the endpoint doesn't exist
    // in the server instance.
    pub no_endpoint: bool,
    pub status_code: Option<u16>,
    pub status_text: Option<String>,
    pub url: String,
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

impl<P> Server<P> {
    pub fn new(base_url: String, endpoints: Vec<Endpoint<P>>) -> Server<P> {
        Server {
            base_url: functions::normalize_url(base_url),
            endpoints,
        }
    }
    pub fn request(&self, method: &str, uri: String) -> Result<ureq::Response, ResponseError> {
        let url = format!("{}{}", self.base_url, uri);
        for endpoint in &self.endpoints {
            // Interate through server's endpoints and only make request if endpoint
            // exists.
            print!("uri: {}\n", uri);
            print!("endoint:{}\n", endpoint.uri);
            if uri == endpoint.uri {
                return match ureq::request(&method.to_uppercase(), &url)
                    .set("Content-Type", "application/json; charset=utf-8")
                    .call()
                {
                    Ok(response) => Ok(response),
                    Err(ureq::Error::Status(code, response)) => Err(ResponseError {
                        no_endpoint: false,
                        status_code: Some(code),
                        status_text: Some(response.status_text().to_string()),
                        url: response.get_url().to_string(),
                    }),
                    // Probably a transport error. Didn't even make it to the
                    // server.
                    _ => Err(ResponseError {
                        no_endpoint: false,
                        status_code: None,
                        status_text: None,
                        url,
                    }),
                };
            }
        }
        // The endpoint isn't even defined on our
        // server object.
        return Err(ResponseError {
            no_endpoint: true,
            status_code: None,
            status_text: None,
            url,
        });
    }
    pub fn get(&self, uri: String) -> Result<ureq::Response, ResponseError> {
        self.request("GET", uri)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn new_server_normalizes_base_url_leaves_slashes_alone() {
        let server = Server::new(
            "https://papi.dev.ocdvlp.com".to_string(),
            vec![Endpoint::new(
                "/opportunities".to_string(),
                "A test endpoint".to_string(),
                HashMap::from([("GET".to_string(), Some("x".to_string()))]),
            )],
        );
        assert_eq!(server.base_url, "https://papi.dev.ocdvlp.com");
    }
    #[test]
    fn new_server_normalizes_base_url_removes_trailing_slash() {
        let server = Server::new(
            "https://papi.dev.ocdvlp.com/".to_string(),
            vec![Endpoint::new(
                "/opportunities".to_string(),
                "A test endpoint".to_string(),
                HashMap::from([("GET".to_string(), Some("x".to_string()))]),
            )],
        );
        assert_eq!(server.base_url, "https://papi.dev.ocdvlp.com");
    }
    #[test]
    fn request_succeeds() {
        let mock_server = MockServer::start();
        // Create a mock on the server.
        mock_server.mock(|when, then| {
            when.method(GET).path("/opportunities/");
            then.status(200)
                .header("content-type", "application/json")
                .body("{\"foo\": true}");
        });
        let server = Server::new(
            mock_server.base_url(),
            vec![Endpoint::new(
                "/opportunities/".to_string(),
                "A test endpoint".to_string(),
                HashMap::from([("GET".to_string(), Some("x".to_string()))]),
            )],
        );
        let resp = server.get("/opportunities/".to_string()).unwrap();
        assert_eq!(resp.status(), 200);
        let json: ureq::serde_json::Value = resp.into_json().unwrap();
        assert_eq!(json["foo"], true);
    }
    #[test]
    fn request_fails_bad_status_code() {
        let mock_server = MockServer::start();
        // Create a mock on the server.
        mock_server.mock(|when, then| {
            when.method(GET).path("/opportunities/");
            then.status(404).header("content-type", "application/json");
        });
        let server = Server::new(
            mock_server.base_url(),
            vec![Endpoint::new(
                "/foo/".to_string(),
                "A test endpoint".to_string(),
                HashMap::from([("GET".to_string(), Some("x".to_string()))]),
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
