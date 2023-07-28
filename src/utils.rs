use regex::Regex;
pub fn validate_url(url: &str) -> bool {
    Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)")
    .unwrap().is_match(url)
}
