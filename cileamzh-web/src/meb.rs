use crate::{HttpRequest, HttpResponse};

pub trait DealHttp {
    fn read_http(&mut self) -> std::io::Result<HttpRequest>;
    fn write_http(self, res: HttpResponse) -> std::io::Result<()>;
}

pub trait SplitByArr {
    // fn split_by_arr(&self, p: Vec<>) {}
}
