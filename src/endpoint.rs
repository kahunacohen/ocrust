fn normalize_uri(uri: &str) -> &str {
    if !uri.ends_with("/") {
        return format!("{}/", uri);
    } else {
        return uri.to_string();
    }
}

pub struct Endpoint <'a> {
    name: String,
    uri: &'a str,
    methods: Vec<String>,
}
impl Endpoint <'_> {
    fn new(name: String, uri: &str, methods: Vec<String>) -> Endpoint {
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
            "/foo",
            vec!["GET".to_string()],
        );
        assert_eq!(e.uri, "/foo/");
    }

    #[test]
    fn implements_method() {
        let e = Endpoint {
            name: "foo".to_string(),
            uri: "/foo",
            methods: vec!["GET".to_string()],
        };
        assert_eq!(e.implements_method("GET".to_string()), true);
    }

    #[test]
    fn doesnt_implement_method() {
        let e = Endpoint {
            name: "foo".to_string(),
            uri: "/foo",
            methods: vec!["GET".to_string()],
        };
        assert_eq!(e.implements_method("POST".to_string()), false);
    }
}
