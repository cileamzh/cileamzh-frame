use crate::{
    meb::{HttpRead, HttpWrite},
    HttpRequest,
};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::spawn,
};

pub struct HttpServer {}
impl HttpServer {
    pub fn new() -> Self {
        HttpServer {}
    }
    pub fn listen(path: &str) -> std::io::Result<()> {
        let lst = TcpListener::bind(path)?;
        for stream in lst.incoming() {
            let stream = stream?;
            spawn(move || handle_stream(stream));
        }
        Ok(())
    }
}

impl HttpRead for TcpStream {
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
}

impl HttpWrite for TcpStream {
    fn write_http(mut self, res: crate::HttpResponse) -> std::io::Result<()> {
        self.write(res.get_header().as_bytes())?;
        self.write(res.get_binary())?;
        self.write_all(res.get_body().as_bytes())?;
        self.flush()?;
        Ok(())
    }
}

fn handle_stream(stream: TcpStream) {}
