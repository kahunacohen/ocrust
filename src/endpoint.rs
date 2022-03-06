fn normalize_uri(uri: String) -> String {
    let mut ret = uri.clone();
    if !uri.ends_with("/") {
        ret = format!("{}/", ret);
    }
    if !uri.starts_with("/") {
        ret = format!("/{}", ret);
    }
    ret
}

pub struct Endpoint {
    uri: String,
    methods: Vec<String>,
}
impl Endpoint {
    fn new(uri: String, methods: Vec<String>) -> Endpoint {
        Endpoint {
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
    fn new_normalizes_uri() {
        let e = Endpoint::new(
            "foo".to_string(),
            vec!["GET".to_string()],
        );
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
