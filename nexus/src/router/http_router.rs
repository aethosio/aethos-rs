// Portions of this code was inpired by the 
// rust router implementation. https://github.com/iron/router


use super::{Router, Handler};

use recognizer::Router as Recognizer;


/**
 * Public types
 */
pub struct HttpRouter(inner::HttpRouter);
pub struct HttpRouterBuilder(inner::HttpRouterBuilder);

/**
 * Private implementation
 */
mod inner {
pub struct HttpRouter {
}  
impl HttpRouter {
}
pub struct HttpRouterBuilder {

}
impl HttpRouterBuilder {
  pub fn new() -> HttpRouterBuilder {
    HttpRouterBuilder{}
  }
}
}

/**
 * Public proxies for newtypes 
 */
impl HttpRouter {
  pub fn builder() -> HttpRouterBuilder {
    HttpRouterBuilder(inner::HttpRouterBuilder::new())
  }
}

impl HttpRouterBuilder {
  pub fn get<G: AsRef<str>, H: Handler, R: AsRef<str>>(self, glob: G, handler: H, route_id: R) -> HttpRouterBuilder {
    unimplemented!();
  }

  pub fn build(self) -> Box<HttpRouter> {
    unimplemented!();
  }
}

impl Router for HttpRouter {

}
