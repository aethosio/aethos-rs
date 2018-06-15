pub mod http_router;

pub use self::http_router::*;

use futures::future::Future;

use http::{Request, Response};
use std::error::Error;

pub trait Router {
}

pub type FutureResponse = Box<Future<Item = Box<Response<()>>, Error=Error>>;

pub trait Handler: Send + Sync + 'static {
  // TODO Haven't decided which is best; return a future, or pass in 
  // a pre-constructed Response (express style), or make this a 
  // blocking function that returns a Response.
  fn handle(&self, req: &Request<()>) -> FutureResponse;
}

// Implement Handler for a function
impl <F> Handler for F 
  where F: Send + Sync + 'static + Fn() -> FutureResponse {
  fn handle(&self, req: &Request<()>) -> FutureResponse {
    (*self)()
  }
}
