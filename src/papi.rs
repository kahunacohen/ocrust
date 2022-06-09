use std::collections::HashMap;

use crate::endpoint::Endpoint;
use crate::server::Server;


pub fn get_server(base_url: String) -> Server<P, 'a> {
    Server::new(
        base_url,
        vec![Endpoint::new(
            "/search/createSearchToken/",
            "Gets a new algolia search token",
            HashMap::from([("GET", None)]),
        )],
    )
}
