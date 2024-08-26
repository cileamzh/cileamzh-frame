use std::collections::HashSet;

use crate::Body;

/// struct HttpResponse is used to represent a httpresponse
pub struct HttpResponse<T = Vec<u8>> {
    pub status: String,
    header: HashSet<(String, String)>,
    pub body: T,
}

impl<T: Body + Default> HttpResponse<T> {
    pub fn new() -> Self {
        HttpResponse {
            status: String::new(),
            header: HashSet::new(),
            body: Default::default(),
        }
    }

    // pub fn from(http_binary: Vec<u8>) -> Self {

    // }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.header.insert((key.to_owned(), value.to_owned()));
    }

    pub fn set_cookie(&mut self, key: &str, value: &str) {
        self.header
            .insert(("Set-Cookie".to_owned(), format!("{}={}", key, value)));
    }
    pub fn join_header(&self) -> String {
        let mut header = format!(
            "{}\r\nContent-Length: {}\r\n",
            self.status,
            self.body.body_len()
        );
        for (key, value) in &self.header {
            header.push_str(&format!("{}: {}\r\n", key, value));
        }
        header.push_str("\r\n");
        header
    }
}
