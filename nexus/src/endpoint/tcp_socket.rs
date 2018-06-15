use std::net::{ToSocketAddrs};
use super::Endpoint;

/**
 * Public types
 */
pub struct TCPSocket(inner::TCPSocket);
pub struct TCPSocketBuilder(inner::TCPSocketBuilder);

/**
 * Private implementation
 */
mod inner {
pub struct TCPSocket {
}  
impl TCPSocket {
}
pub struct TCPSocketBuilder {

}
impl TCPSocketBuilder {
  pub fn new() -> TCPSocketBuilder {
    TCPSocketBuilder{}
  }
  pub fn bind<A: super::ToSocketAddrs>(&mut self, addrs: A) {
    unimplemented!();
  }
}
}

/**
 * Public proxies for newtypes 
 */
impl TCPSocket {
  pub fn builder() -> TCPSocketBuilder {
    TCPSocketBuilder(inner::TCPSocketBuilder::new())
  }
}

impl TCPSocketBuilder {
  pub fn bind<A: ToSocketAddrs>(self, addr: A) -> TCPSocketBuilder {
    let TCPSocketBuilder(mut inner) = self;
    inner.bind(addr);
    TCPSocketBuilder(inner)
  }

  pub fn build(self) -> Box<TCPSocket> {
    unimplemented!();
  }
}

impl Endpoint for TCPSocket {

}
