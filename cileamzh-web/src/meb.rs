use crate::{HttpRequest, HttpResponse};
pub trait ToVec {
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
