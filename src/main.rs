struct Endpoint {
    name: &'static str,
    uri: &'static str,
    methods: Vec<&'static str>,
}
impl Endpoint {
    fn implements_method(&self, method: &str) -> bool {
        self.methods.contains(&method)
    }
}
#[cfg(test)]
#[test]
fn implements_method() {
    let e = Endpoint {
        name: "foo",
        uri: "/foo",
        methods: vec!["GET"],
    };
    assert_eq!(e.implements_method("GET"), true);
}
#[test]
fn doesnt_implement_method() {
    let e = Endpoint {
        name: "foo",
        uri: "/foo",
        methods: vec!["GET"],
    };
    assert_eq!(e.implements_method("POST"), false);
}

fn main() {
    println!("Hello world!")
}
