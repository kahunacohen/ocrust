use regex::Regex;

pub fn normalize_uri(uri: &String) -> String {
    let re = Regex::new(r"^/|/$").unwrap();
    format!("/{}/", re.replace_all(uri, ""))
}