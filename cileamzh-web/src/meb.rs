use crate::{HttpRequest, HttpResponse};

pub trait DealHttp {
    fn read_http(&mut self) -> std::io::Result<HttpRequest>;
    fn write_http(self, res: HttpResponse) -> std::io::Result<()>;
}
// HttpBody is a Trait of body if you have it you will be treated as HttpBody
pub trait Body {
    // body_len back it's length
    fn body_len(&self) -> usize;
}

impl Body for String {
    fn body_len(&self) -> usize {
        self.len()
    }
}

impl Body for Vec<u8> {
    fn body_len(&self) -> usize {
        self.len()
    }
}
