use super::Protocol;

/**
 * Public types
 */
pub struct HttpServer(inner::HttpServer);

/**
 * Private implementation
 */
mod inner {
pub struct HttpServer {
}  
impl HttpServer {
  pub fn new() -> HttpServer {
    HttpServer{}
  }
}
}

/**
 * Public proxies for newtypes 
 */
impl HttpServer {
  pub fn new() -> HttpServer {
    HttpServer(inner::HttpServer::new())
  }
}

impl Protocol for HttpServer {

}