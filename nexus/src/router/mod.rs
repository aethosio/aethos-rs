pub mod http_router;

pub use self::http_router::*;

use super::protocol::{Request, Response};

use futures::future::Future;

use std::error::Error;

pub trait Router {
}

pub type FutureResponse = Box<Future<Item = Box<Response>, Error=Error>>;

pub trait Handler: Send + Sync + 'static {
  fn handle(&self, req: &Request) -> FutureResponse;
}

// Implement Handler for a function
impl <F> Handler for F 
  where F: Send + Sync + 'static + Fn() -> FutureResponse {
  fn handle(&self, req: &Request) -> FutureResponse {
    (*self)()
  }
}
