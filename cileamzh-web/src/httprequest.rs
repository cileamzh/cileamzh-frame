use std::collections::HashMap;

use crate::Body;

pub struct HttpRequest<T = Vec<u8>> {
    params: HashMap<String, String>,
    pub path: String,
    pub method: String,
    pub protocol: String,
    header: HashMap<String, String>,
    pub body: T,
}
impl<T: Body + Default> HttpRequest<T> {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
            path: String::new(),
            method: String::new(),
            protocol: String::new(),
            header: HashMap::new(),
            body: T::default(),
        }
    }

    pub fn set_header(&mut self, k: String, v: String) {
        self.header.insert(k, v);
    }

    pub fn set_params(&mut self, k: String, v: String) {
        self.params.insert(k, v);
    }
    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.header.get(key)
    }
    pub fn get_params(&self, k: &str) -> Option<&String> {
        self.params.get(k)
    }

    pub fn join_header(&self) {}
}
