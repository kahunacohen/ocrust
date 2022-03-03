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
    assert_eq!(e.implements_method(String::from("GET")), true);
}
#[test]
fn doesnt_implement_method() {
    let e = Endpoint {
        name: String::from("foo"),
        uri: String::from("/foo"),
        methods: vec![String::from("GET")],
    };
    assert_eq!(e.implements_method(String::from("POST")), false);
}

fn main() {
    println!("Hello world!")
}
