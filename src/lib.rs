//! This is the documentation for the `parse_url` crate.
//!
//! The `parse_url` crate is used for breaking down a URL into indivdual parts.
//!
//! # Examples
//!
//! ```
//! use parse_url::*;
//! let url = String::from("https://www.example.com/en-US/page/sub/?pre=2&foo=bar#fuzz");
//! let parts = parse_url(&url);
//!
//! assert_eq!(Some("https"), parts.protocol);
//! assert_eq!(Some("www.example.com"), parts.host);
//! assert_eq!(Some("en-US/page/sub/"), parts.path);
//! assert_eq!(Some("pre=2&foo=bar"), parts.search);
//! assert_eq!(("pre", "2"), parts.params[0]);
//! assert_eq!(("foo", "bar"), parts.params[1]);
//! assert_eq!(Some("fuzz"), parts.fragment);
//! ```

mod utils;

pub struct URLParts<'a> {
    pub protocol: Option<&'a str>,
    pub host: Option<&'a str>,
    pub path: Option<&'a str>,
    pub search: Option<&'a str>,
    pub fragment: Option<&'a str>,
    pub params: Vec<(&'a str, &'a str)>
}

pub fn get_protocol(url: &str) -> Option<&str> {
    let protocol: Option<&str> = utils::truncate(&url, "://", 0);

    if protocol.is_some() {
        let protocol_type = protocol.unwrap();
        match protocol_type {
            "http" | "https" | "ftp" => return protocol,
            _ => return None
        }
    }

    None
}

pub fn get_host(url: &str) -> Option<&str> {
    let result = utils::truncate(&url, "://", 1);

    if result.is_none() {
        return utils::truncate(&url, "/", 0)
    }

    utils::truncate(&result.unwrap(), "/", 0) // strip path
}

pub fn get_path(url: &str) -> Option<&str> {
    let result = utils::truncate(&url, "://", 1);
    let mut path: Option<&str>;

    if result.is_none() {
        path = utils::truncate(&url, "/", 1);
    } else {
        path = utils::truncate(&result.unwrap(), "/", 1);
    }

    if path.is_some() {
        path = utils::truncate(&path.unwrap(), "?", 0);
        return utils::truncate(&path.unwrap(), "#", 0);
    }

    None
}

pub fn get_search_string(url: &str) -> Option<&str> {
    let search: Option<&str> = utils::truncate(&url, "?", 1);

    if !search.is_none() {
        return utils::truncate(&search.unwrap(), "#", 0) // strip hash
    }

    None
}

pub fn get_fragment(url: &str) -> Option<&str> {
    utils::truncate(&url, "#", 1)
}

pub fn get_params(url: &str) -> Vec<(&str, &str)> {
    let search = get_search_string(&url);
    let mut result: Vec<(&str, &str)> = vec![];

    if !search.is_none() {
        let params: Vec<&str> = search.unwrap().split("&").collect();

        for p in params {
            let param: Vec<&str> = p.splitn(2, "=").collect();

            if param.len() == 2 {
                let k = param[0].clone();
                let v = param[1].clone();
                result.push((k, v));
            }
        }
    }

    result
}

pub fn parse_url(url: &str) -> URLParts {
    let url: &str = url.trim();

    URLParts {
        protocol: get_protocol(&url),
        host: get_host(&url),
        path: get_path(&url),
        search: get_search_string(&url),
        fragment: get_fragment(&url),
        params: get_params(&url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url() {
        let url = String::from("https://www.example.com/en-US/page/sub/?pre=2&foo=bar#fuzz");
        let parts = parse_url(&url);

        assert_eq!(Some("https"), parts.protocol);
        assert_eq!(Some("www.example.com"), parts.host);
        assert_eq!(Some("en-US/page/sub/"), parts.path);
        assert_eq!(Some("pre=2&foo=bar"), parts.search);
        assert_eq!(("pre", "2"), parts.params[0]);
        assert_eq!(("foo", "bar"), parts.params[1]);
        assert_eq!(Some("fuzz"), parts.fragment);
    }

    #[test]
    fn path() {
        let url = String::from("https://www.example.com/en-US/page/sub/");
        let parts = parse_url(&url);

        assert_eq!(Some("https"), parts.protocol);
        assert_eq!(Some("www.example.com"), parts.host);
        assert_eq!(Some("en-US/page/sub/"), parts.path);
        assert_eq!(None, parts.search);
        assert_eq!(0, parts.params.len());
        assert_eq!(None, parts.fragment);
    }

    #[test]
    fn fragment() {
        let url = String::from("https://www.example.com/en-US/page/sub/#fuzz");
        let parts = parse_url(&url);

        assert_eq!(Some("https"), parts.protocol);
        assert_eq!(Some("www.example.com"), parts.host);
        assert_eq!(Some("en-US/page/sub/"), parts.path);
        assert_eq!(None, parts.search);
        assert_eq!(0, parts.params.len());
        assert_eq!(Some("fuzz"), parts.fragment);
    }

    #[test]
    fn host() {
        let url = String::from("https://www.example.com/");
        let parts = parse_url(&url);

        assert_eq!(Some("https"), parts.protocol);
        assert_eq!(Some("www.example.com"), parts.host);
        assert_eq!(None, parts.path);
        assert_eq!(None, parts.search);
        assert_eq!(0, parts.params.len());
        assert_eq!(None, parts.fragment);
    }

    #[test]
    fn protocol_invalid() {
        let url = String::from("foo://www.example.com/en-US/page/sub/");
        let parts = parse_url(&url);

        assert_eq!(None, parts.protocol);
        assert_eq!(Some("www.example.com"), parts.host);
        assert_eq!(Some("en-US/page/sub/"), parts.path);
        assert_eq!(None, parts.search);
        assert_eq!(0, parts.params.len());
        assert_eq!(None, parts.fragment);
    }

    #[test]
    fn protocol_none() {
        let url = String::from("www.example.com/en-US/page/sub/");
        let parts = parse_url(&url);

        assert_eq!(None, parts.protocol);
        assert_eq!(Some("www.example.com"), parts.host);
        assert_eq!(Some("en-US/page/sub/"), parts.path);
        assert_eq!(None, parts.search);
        assert_eq!(0, parts.params.len());
        assert_eq!(None, parts.fragment);
    }

    #[test]
    fn params() {
        let url = String::from("https://www.example.com/en-US/page/sub/?pre=2&foo=bar#fuzz");
        let parts = parse_url(&url);

        assert_eq!(Some("https"), parts.protocol);
        assert_eq!(Some("www.example.com"), parts.host);
        assert_eq!(Some("en-US/page/sub/"), parts.path);
        assert_eq!(Some("pre=2&foo=bar"), parts.search);
        assert_eq!(("pre", "2"), parts.params[0]);
        assert_eq!(("foo", "bar"), parts.params[1]);
        assert_eq!(Some("fuzz"), parts.fragment);
    }

    #[test]
    fn white_space() {
        let url = String::from(" https://www.example.com/en-US/page/sub/#fuzz  ");
        let parts = parse_url(&url);

        assert_eq!(Some("https"), parts.protocol);
        assert_eq!(Some("www.example.com"), parts.host);
        assert_eq!(Some("en-US/page/sub/"), parts.path);
        assert_eq!(None, parts.search);
        assert_eq!(0, parts.params.len());
        assert_eq!(Some("fuzz"), parts.fragment);
    }
}
