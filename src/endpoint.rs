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
        .unwrap_or(&s)
        .to_owned()
}

// fn normalize_uri2(uri: &str) -> String {
//     match uri.strip_suffix("/") {
//         Some(s) => println!("hiiiiiiiiiiii"),
//         None => {

//         },
//     }
//     "/foo/".to_string()
// }
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn endpoint_strips_slashes() {
        let e = Endpoint::new(String::from("/foo/"), vec!["GET".to_string()]);
        assert_eq!(e.uri, "foo");
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
