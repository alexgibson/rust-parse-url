pub fn truncate<'a>(url: &'a str, separator: &'a str, index: usize) -> Option<&'a str> {
    let v: Vec<&str> = url.splitn(2, &separator).collect();
    let result: &str = match v.get(index) {
        Some(x) => x,
        None => "None"
    };

    if !result.is_empty() && result != "None" {
        return Some(result)
    }

    return None
}
