/// struct HttpResponse is used to represent a httpresponse
pub struct HttpResponse {
    pub protocal: String,
    pub status: String,
    header: String,
    body: String,
    binary: Vec<u8>,
}

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse {
            protocal: String::new(),
            status: String::new(),
            header: String::new(),
            body: String::new(),
            binary: Vec::new(),
        }
    }
}
