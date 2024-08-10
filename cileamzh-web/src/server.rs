use std::{
    collections::HashMap,
    fs::{self},
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    sync::Arc,
    thread::spawn,
};

use crate::{Handler, HttpRequest, HttpResponse, Middleware};

pub struct HttpServer {
    routers: Arc<HashMap<(String, String), Arc<Handler>>>,
    middlewares: Arc<Vec<Arc<Middleware>>>,
    static_route: Arc<Vec<(String, String)>>,
}

impl HttpServer {
    pub fn new() -> Result<Self> {
        Ok(HttpServer {
            routers: Arc::new(HashMap::new()),
            middlewares: Arc::new(Vec::new()),
            static_route: Arc::new(Vec::new()),
        })
    }

    pub fn listen(self, host: &str) -> Result<()> {
        let listener: TcpListener = TcpListener::bind(host)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let routers = Arc::clone(&self.routers);
                    let middlewares = Arc::clone(&self.middlewares);
                    let static_route = Arc::clone(&self.static_route);
                    spawn(move || {
                        if let Err(e) = handle_stream(stream, routers, middlewares, static_route) {
                            eprintln!("Error handling stream: {}", e);
                        }
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
        Ok(())
    }

    pub fn add_router<F>(&mut self, method: &str, path: &str, handler: F)
    where
        F: Fn(&mut HttpRequest, &mut HttpResponse) + Send + Sync + 'static,
    {
        Arc::get_mut(&mut self.routers)
            .unwrap()
            .insert((method.to_string(), path.to_string()), Arc::new(handler));
    }

    pub fn add_middleware<F>(&mut self, middleware: F)
    where
        F: Fn(&mut HttpRequest, &mut HttpResponse) + Send + Sync + 'static,
    {
        Arc::get_mut(&mut self.middlewares)
            .unwrap()
            .push(Arc::new(middleware));
    }

    pub fn add_get<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&mut HttpRequest, &mut HttpResponse) + Send + Sync + 'static,
    {
        self.add_router("GET", path, handler);
    }

    pub fn add_post<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&mut HttpRequest, &mut HttpResponse) + Send + Sync + 'static,
    {
        self.add_router("POST", path, handler);
    }
    pub fn add_static_dir(&mut self, path: &str, dir: &str) {
        Arc::get_mut(&mut self.static_route)
            .unwrap()
            .push((path.to_owned(), dir.to_owned()));
    }
}
fn handle_stream(
    mut stream: TcpStream,
    route: Arc<HashMap<(String, String), Arc<Handler>>>,
    middlewares: Arc<Vec<Arc<Middleware>>>,
    static_route: Arc<Vec<(String, String)>>,
) -> Result<()> {
    let req_str = "www";
    let mut req = HttpRequest::from(req_str.to_owned())?;
    let mut res = HttpResponse::new();

    if static_route.len() >= 1 {
        for (path, dir) in static_route.iter() {
            if path.len() < req.get_path().len() {
                if req.get_path().get(..path.len()).unwrap() == path
                    && req.get_path().chars().nth(path.len()).unwrap() == '/'
                {
                    let file_str = format!(
                        "{}/{}",
                        dir,
                        req.get_path()
                            .get((path.len() + 1)..)
                            .unwrap_or("not_found")
                    );
                    if Path::new(&file_str).exists() {
                        let contents = fs::read(&file_str).unwrap();
                        res.set_header("Content-Type", get_mime_type(&file_str));
                        res.set_body(&String::from_utf8_lossy(&contents));
                        stream.write(res.get_header().as_bytes())?;
                        stream.write_all(&contents)?;
                        stream.flush()?
                    } else {
                        res.set_body("file not found");
                        stream.write(res.get_header().as_bytes())?;
                        stream.write_all("file not found".as_bytes())?;
                        stream.flush()?
                    }
                }
            }
        }
    }

    if middlewares.len() >= 1 {
        for middleware in middlewares.iter() {
            middleware(&mut req, &mut res);
        }
    }

    let key = (req.get_method().to_string(), req.get_path().to_string());
    if let Some(handler) = route.get(&key) {
        handler(&mut req, &mut res);
        stream.write_all(res.get_header().as_bytes())?;
        stream.flush()?;
    } else {
        res.set_body("404 Not Found");
        res.set_header("Content-Type", "text/plain");
        stream.write(res.get_header().as_bytes())?;
        stream.write(&res.get_binary())?;
        stream.flush()?;
    }

    Ok(())
}

fn get_mime_type(file_path: &str) -> &str {
    match Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("mp4") => "video/mp4",
        Some("mp3") => "audio/mpeg",
        Some("ogg") => "audio/ogg",
        Some("wav") => "audio/wav",
        Some("pdf") => "application/pdf",
        _ => "application/octet-stream",
    }
}

impl HttpReader for TcpStream {
    fn read_to_httpstr(mut stream: &TcpStream) -> Result<String> {
        let mut buf = [0; 512];
        let mut all_bytes: Vec<u8> = vec![];
        loop {
            let read = stream.read(&mut buf)?;
            if read == 0 {
                break;
            }
            all_bytes.append(&mut buf[..read].to_vec());
            if read < buf.len() {
                break;
            }
        }
        // println!("{}", &result);
        Ok("show".to_owned())
    }
}

pub trait HttpReader {
    fn read_to_httpstr(mut stream: &TcpStream) -> Result<String> {
        let mut buf = [0; 512];
        let mut all_bytes: Vec<u8> = vec![];
        loop {
            let read = stream.read(&mut buf)?;
            if read == 0 {
                break;
            }
            all_bytes.append(&mut buf[..read].to_vec());
            if read < buf.len() {
                break;
            }
        }
        // println!("{}", &result);
        Ok("show".to_owned())
    }
    fn read_to_map() -> HashMap<String, String> {
        HashMap::new()
    }
}
