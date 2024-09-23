use std::fmt::Error;

pub struct HttpRequest {
    pub params: String,
    pub path: String,
    pub method: String,
    pub protocol: String,
    header: String,
    body: String,
    pub binary: Vec<u8>,
}
impl HttpRequest {
    pub fn new() -> Self {
        Self {
            params: String::new(),
            path: String::new(),
            method: String::new(),
            protocol: String::new(),
            header: String::new(),
            body: String::new(),
            binary: Vec::new(),
        }
    }

    pub fn from(_buf: Vec<u8>) -> Self {
        let res = Self {
            params: String::new(),
            path: String::new(),
            method: String::new(),
            protocol: String::new(),
            header: String::new(),
            body: String::new(),
            binary: Vec::new(),
        };
        res
    }

    pub fn cookies(&mut self, cookie: &str) {
        self.header.push_str("Cookie: ");
        self.header.push_str(cookie);
        self.header.push_str("\r\n");
    }

    pub fn push_header(&mut self, header: &str) {
        self.header.push_str(header);
        self.header.push_str("\r\n");
    }

    pub fn body(&mut self, body: &str) {
        self.body.push_str(body);
    }

    pub fn formot(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.append(&mut self.method.to_vec_u8());
        buf.append(&mut " ".to_vec_u8());
        buf.append(&mut self.path.to_vec_u8());
        buf.append(&mut self.params.to_vec_u8());
        buf.append(&mut " ".to_vec_u8());
        buf.append(&mut self.protocol.to_vec_u8());
        buf.append(&mut "\r\n".to_vec_u8());
        buf.append(&mut self.header.to_vec_u8());
        buf.append(&mut "\r\n\r\n".to_vec_u8());
        buf.append(&mut self.body.to_vec_u8());
        buf.append(&mut self.binary.to_vec());
        buf
    }
}

trait ToVec {
    fn to_vec_u8(&self) -> Vec<u8>;
}

impl ToVec for String {
    fn to_vec_u8(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ToVec for &str {
    fn to_vec_u8(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

fn split(a: Vec<u8>, p: Vec<u8>) -> Result<(), Error> {
    panic!("{}", "len is wrong");
    Ok(())
}
