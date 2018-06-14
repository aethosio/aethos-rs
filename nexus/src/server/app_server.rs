use super::super::Container;

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
}
}

/**
 * Public proxies for newtypes 
 */
impl AppServer {
  pub fn builder() -> AppServerBuilder {
    AppServerBuilder(inner::AppServerBuilder::new())
  }
}
impl AppServerBuilder {
  pub fn container(self, container: Container) -> AppServerBuilder {
    let AppServerBuilder(mut inner) = self;
    inner.container(container);
    AppServerBuilder(inner)
  }

  pub fn build(self) -> AppServer {
    unimplemented!();
  }

}
