use crate::{meb::DealHttp, ware::Ware, HttpRequest, HttpResponse};
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    path::Path,
};
/// HttpServer represent a server for the communication with Http protocal
///
/// # Examples
///
/// ```
/// let mut server=HttpServer::new();
/// ```
pub struct HttpServer {
    lst: Vec<TcpListener>,
    warelist: Vec<Ware>,
}
impl HttpServer {
    ///Create a httpserver
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut server=HttpServer::new();
    /// ```
    pub fn new() -> Self {
        HttpServer {
            lst: Vec::new(),
            warelist: Vec::new(),
        }
    }
    ///Listen a host and start your server.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let server=HttpServer::new();
    /// server.listen("localhost",8080)?;
    /// ```
    ///
    pub fn listen(&mut self, mut host: &str, port: u64) -> std::io::Result<()> {
        if host == "localhost" {
            host = "127.0.0.1"
        }
        if self.lst.len() == 0 {
            let lst = TcpListener::bind(format!("{}:{}", host, port))?;
            self.lst.push(lst);
            println!("server listening {}:{}", host, port);
            for stream in self.lst[0].incoming() {
                let stream = stream?;
                self.handle_stream(stream).unwrap();
            }
            Ok(())
        } else {
            println!("Server can't listen twice");
            Ok(())
        }
    }
    ///Quickly listen localhost
    ///
    /// #Example
    /// ```rust
    /// server.localhost(8080)?;
    /// ```
    pub fn localhost(&mut self, port: u64) -> std::io::Result<()> {
        self.listen("localhost", port)?;
        Ok(())
    }

    ///Mount you middleware and router
    ///
    /// #Examples
    ///
    /// ```rust
    /// server.mount(your_midware);
    /// server.mount(your_router);
    /// ```
    pub fn mount(&mut self, mid: Ware) {
        match mid {
            Ware::Router(router) => self.warelist.push(Ware::Router(router)),
            Ware::Middleware(middleware) => self.warelist.push(Ware::Middleware(middleware)),
        }
    }

    ///Set a dir as static dir.
    ///
    /// #Exmaple
    ///
    /// ```rust
    /// server.set_static_dir(route_path,dir_path);
    /// ```
    pub fn set_static_dir(_route_path: &str, dir_path: &str) {
        if Path::new(dir_path).is_dir() {
        } else {
        }
    }

    fn _through_ware(&self, mut req: HttpRequest, mut res: HttpResponse) {
        let i: usize = 0;
        loop {
            if i >= self.warelist.len() {
                break;
            }
            match &self.warelist[i] {
                Ware::Router(router) => {
                    let (m, p) = (&req.method, &req.path);
                    if (m, p) == (&router.0, &router.1) {
                        res = router.2(req, res);
                        print!("{}", res.status);
                        break;
                    }
                }
                Ware::Middleware(middleware) => {
                    (req, res) = middleware(req, res);
                }
            }
        }
    }
    fn handle_stream(&self, mut stream: TcpStream) -> std::io::Result<()> {
        let mut buffer = [0; 512];
        let mut binary_http: Vec<u8> = Vec::new();
        loop {
            let len = stream.read(&mut buffer)?;
            binary_http.append(&mut buffer.to_vec());
            if len < buffer.len() {
                break;
            }
        }
        let req: HttpRequest<Vec<u8>> = HttpRequest::new();
        let res: HttpResponse<Vec<u8>> = HttpResponse::new();
        self._through_ware(req, res);
        Ok(())
    }
}
impl DealHttp for TcpStream {
    fn read_http(&mut self) -> std::io::Result<HttpRequest> {
        let mut buf: [u8; 512] = [0; 512];
        let mut binary: Vec<u8> = vec![];
        loop {
            let readsize = self.read(&mut buf)?;
            binary.append(&mut buf.to_vec());
            if readsize < buf.len() {
                break;
            }
        }
        let res = HttpRequest::new();
        Ok(res)
    }
    fn write_http(self, _res: crate::HttpResponse) -> std::io::Result<()> {
        Ok(())
    }
}
