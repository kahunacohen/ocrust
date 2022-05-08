use std::collections::HashMap;

use crate::endpoint::Endpoint;
use crate::functions;

pub struct Server<K, V> {
    pub base_url: String,
    pub endpoints: Vec<Endpoint<K, V>>,
}

impl<K, V> Server<K, V> {
    pub fn new(base_url: String, endpoints: Vec<Endpoint<K, V>>) -> Server<K, V> {
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
                HashMap::from([("Mercury".to_string(), "0.4".to_string())]),
            )],
        );
        assert_eq!(server.base_url, "https://papi.dev.ocdvlp.com/opportunities");
    }
}
