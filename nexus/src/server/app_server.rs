use super::super::Container;
use super::super::router::Router;

/**
 * Public types
 */
pub struct AppServer(inner::AppServer);
pub struct AppServerBuilder(inner::AppServerBuilder);

/**
 * Private implementation
 */
mod inner {
pub struct AppServer {
}
pub struct AppServerBuilder {

}
impl AppServerBuilder {
  pub fn new() -> AppServerBuilder {
    AppServerBuilder {}
  }

  pub fn container(&mut self, container: super::Container) {
    unimplemented!();
  }

  pub fn router(&mut self, router: Box<super::Router>) {
    unimplemented!();
  }
}
}

/**
 * Public proxies for newtypes 
 */
impl AppServer {
  pub fn builder() -> AppServerBuilder {
    AppServerBuilder(inner::AppServerBuilder::new())
  }
  pub fn run(self) -> AppServer {
    unimplemented!();
  }

  pub fn results(&self) -> Result<i32, &'static str> {
    unimplemented!();
  }
}
impl AppServerBuilder {
  pub fn container(self, container: Container) -> AppServerBuilder {
    let AppServerBuilder(mut inner) = self;
    inner.container(container);
    AppServerBuilder(inner)
  }

  pub fn router(self, router: Box<Router>) -> AppServerBuilder {
    let AppServerBuilder(mut inner) = self;
    inner.router(router);
    AppServerBuilder(inner)
  }

  pub fn build(self) -> AppServer {
    unimplemented!();
  }

}
