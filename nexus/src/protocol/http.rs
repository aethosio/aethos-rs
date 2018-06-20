use super::Protocol;
use super::super::endpoint::Endpoint;

/**
 * Public types
 */
pub struct Server(inner::Server);
pub struct ServerBuilder(inner::ServerBuilder);

/**
 * Private implementation
 */
mod inner {
pub struct Server {
}  
impl Server {
}
pub struct ServerBuilder {
}
impl ServerBuilder {
  pub fn new() -> ServerBuilder {
    ServerBuilder{}
  }
  pub fn listen(&mut self, endpoint: Box<super::Endpoint>) {
    unimplemented!();
  }
}
}

/**
 * Public proxies for newtypes 
 */
impl Server {
  pub fn builder() -> ServerBuilder {
    ServerBuilder(inner::ServerBuilder::new())
  }
}
impl ServerBuilder {
  pub fn listen(self, endpoint: Box<Endpoint>) -> ServerBuilder {
    let ServerBuilder(mut inner) = self;
    inner.listen(endpoint);
    ServerBuilder(inner)
  }
  pub fn build(self) -> Box<Server> {
    unimplemented!();
  }
}

impl Protocol for Server {

}