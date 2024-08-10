use std::collections::HashMap;

pub struct HttpResponse {
    status: String,
    headers: HashMap<String, String>,
    body: String,
    binary: Vec<u8>,
}

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse {
            status: "HTTP/1.1 200 OK".to_owned(),
            headers: HashMap::new(),
            body: String::new(),
            binary: Vec::new(),
        }
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = body.to_string();
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn set_binary(&mut self, values: Vec<u8>) {
        self.binary = values;
    }

    pub fn get_body(&self) -> &str {
        &self.body
    }

    pub fn get_binary(&mut self) -> &Vec<u8> {
        &self.binary
    }

    pub fn get_header(&self) -> String {
        let mut response = format!("{}\r\nContent-Length: {}\r\n", self.status, self.body.len());
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str("\r\n");
        response
    }
}
