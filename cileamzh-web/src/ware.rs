use crate::{HttpRequest, HttpResponse};

///Router is used to route the communication
type Router = (
    String,
    String,
    fn(HttpRequest, HttpResponse) -> HttpResponse,
);
type Middleware = fn(HttpRequest, HttpResponse) -> (HttpRequest, HttpResponse);

/// emum Ware contains two members,Router and MiddleWare
///
///  
pub enum Ware {
    Router(Router),
    Middleware(Middleware),
}
