use crate::{HttpRequest, HttpResponse};

pub struct Router {
    path: String,
    midware: Middleware,
    route: fn(&mut HttpRequest, &mut HttpResponse),
}

pub struct Middleware {}

pub trait HttpRead {
    fn read_http(&mut self) -> std::io::Result<HttpRequest>;
}

pub trait HttpWrite {
    fn write_http(self, res: HttpResponse) -> std::io::Result<()>;
}
