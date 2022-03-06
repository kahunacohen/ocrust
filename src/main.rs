struct Endpoint {
    name: String,
    uri: String,
    methods: Vec<String>,
}
impl Endpoint {
    fn implements_method(&self, method: String) -> bool {
        self.methods.contains(&method)
    }
}
#[cfg(test)]
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

fn main() {
    println!("Hello world!")
}
