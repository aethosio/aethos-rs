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

impl Endpoint for TCPSocket {

}
