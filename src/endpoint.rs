fn normalize_uri(uri: String) -> String {
    if !uri.ends_with("/") {
        return format!("{}/", uri);
    } else {
        return uri;
    }
}

pub struct Endpoint {
    name: String,
    uri: String,
    methods: Vec<String>,
}
impl Endpoint {
    fn new(name: String, uri: String, methods: Vec<String>) -> Endpoint {
        Endpoint {
            name,
            uri: normalize_uri(uri),
            methods,
        }
    }

    fn implements_method(&self, method: String) -> bool {
        self.methods.contains(&method)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_adds_trailing_slash_to_uri() {
        let e = Endpoint::new(
            "foo".to_string(),
            "/foo".to_string(),
            vec!["GET".to_string()],
        );
        assert_eq!(e.uri, "/foo/");
    }

    #[test]
    fn implements_method() {
        let e = Endpoint {
            name: "foo".to_string(),
            uri: "/foo".to_string(),
            methods: vec!["GET".to_string()],
        };
        assert_eq!(e.implements_method("GET".to_string()), true);
    }

    #[test]
    fn doesnt_implement_method() {
        let e = Endpoint {
            name: "foo".to_string(),
            uri: "/foo".to_string(),
            methods: vec!["GET".to_string()],
        };
        assert_eq!(e.implements_method("POST".to_string()), false);
    }
}
