use super::Protocol;
use super::super::endpoint::Endpoint;

/**
 * Public types
 */
pub struct HttpServer(inner::HttpServer);
pub struct HttpServerBuilder(inner::HttpServerBuilder);

/**
 * Private implementation
 */
mod inner {
pub struct HttpServer {
}  
impl HttpServer {
}
pub struct HttpServerBuilder {
}
impl HttpServerBuilder {
  pub fn new() -> HttpServerBuilder {
    HttpServerBuilder{}
  }
  pub fn listen(&mut self, endpoint: Box<super::Endpoint>) {
    unimplemented!();
  }
}
}

/**
 * Public proxies for newtypes 
 */
impl HttpServer {
  pub fn builder() -> HttpServerBuilder {
    HttpServerBuilder(inner::HttpServerBuilder::new())
  }
}
impl HttpServerBuilder {
  pub fn listen(self, endpoint: Box<Endpoint>) -> HttpServerBuilder {
    let HttpServerBuilder(mut inner) = self;
    inner.listen(endpoint);
    HttpServerBuilder(inner)
  }
  pub fn build(self) -> Box<HttpServer> {
    unimplemented!();
  }
}

impl Protocol for HttpServer {

}