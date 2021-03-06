use std::collections::HashMap;

use crate::endpoint::{Endpoint, Payload};
use crate::server::Server;

pub fn get_server(base_url: String) -> Server<Payload> {
    Server::new(
        base_url,
        vec![
            Endpoint::new(
                "/search/createSearchToken/".to_string(),
                "Gets a new algolia search token".to_string(),
                HashMap::from([("GET".to_string(), None)]),
            ),
            Endpoint::new(
                "/user/getabtest/".to_string(),
                "Gets a/b test varients".to_string(),
                HashMap::from([("GET".to_string(), None)]),
            ),
            Endpoint::new(
                "/opportunities/".to_string(),
                "A test endpoint".to_string(),
                HashMap::from([(
                    "GET".to_string(),
                    Some(Payload {
                        data: Some("x".to_string()),
                    }),
                )]),
            ),
        ],
    )
}
