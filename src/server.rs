use crate::endpoint::Endpoint;

pub struct Server {
    pub base_url: String,
    pub endpoints: Vec<Endpoint>,
}

impl Server {
    pub fn new(base_url: String, endpoints: Vec<Endpoint>) -> Server {
        Server {
            base_url,
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
    fn new_server() {
        let s = Server::new(
            "https://papi.dev.ocdvlp.com/opportunities".to_string(),
            vec![Endpoint::new(
                "/opportunities".to_string(),
                vec!["GET".to_string()],
            )],
        );
        let ret = s.get("/opportunities".to_string());
        assert_eq!(ret, "foo");
    }
}
