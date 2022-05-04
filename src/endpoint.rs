pub struct Endpoint {
    pub uri: String,
    pub methods: Vec<String>,
}
impl Endpoint {
    pub fn new(uri: String, methods: Vec<String>) -> Endpoint {
        Endpoint {
            uri: normalize_uri(uri),
            methods,
        }
    }

    pub fn implements_method(&self, method: String) -> bool {
        self.methods.contains(&method)
    }
}

fn normalize_uri(s: String) -> String {
    s.strip_suffix("/")
        .and_then(|s| s.strip_prefix("/"))
        .map(|s| format!("/{s}/"))
        .unwrap_or(s)
        .to_owned()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn endpoint_handles_leading_trailing_slash() {
        let e = Endpoint::new(String::from("/foo/"), vec!["GET".to_string()]);
        assert_eq!(e.uri, "/foo/");
    }
    #[test]
    fn endpoint_handles_no_slashes() {
        let e = Endpoint::new(String::from("foo"), vec!["GET".to_string()]);
        assert_eq!(e.uri, "/foo/");
    }

    #[test]
    fn implements_method() {
        let e = Endpoint {
            uri: "/foo".to_string(),
            methods: vec!["GET".to_string()],
        };
        assert_eq!(e.implements_method("GET".to_string()), true);
    }

    #[test]
    fn doesnt_implement_method() {
        let e = Endpoint {
            uri: "/foo".to_string(),
            methods: vec!["GET".to_string()],
        };
        assert_eq!(e.implements_method("POST".to_string()), false);
    }
}
