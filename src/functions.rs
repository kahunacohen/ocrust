use regex::Regex;

pub fn normalize_uri(uri: &str) -> String {
    let re = Regex::new(r"^/|/$").unwrap();
    format!("/{}/", re.replace_all(uri, ""))
}
pub fn normalize_url(url: String) -> String {
    url.strip_suffix("/").unwrap_or(&url).to_string()
}
