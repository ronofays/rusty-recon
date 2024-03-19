

pub fn clean_url(url: &str) -> &str {
    match url.chars().next() {
        Some('*') | Some('.') if url.len() >= 2 => &url[1..],
        _ => url,
    }
}