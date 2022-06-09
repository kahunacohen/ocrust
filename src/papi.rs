use std::collections::HashMap;

use crate::endpoint::{Endpoint, Payload};
use crate::server::Server;

pub fn get_server(base_url: String) -> Server<Payload> {
    Server::new(
        base_url,
        vec![Endpoint::new(
            "/search/createSearchToken/".to_string(),
            "Gets a new algolia search token".to_string(),
            HashMap::from([("GET".to_string(), None)]),
        )],
    )
}
