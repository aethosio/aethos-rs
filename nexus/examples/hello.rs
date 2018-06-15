extern crate nexus;

use nexus::{Container, Channel, AppServer};
use nexus::protocol::{HttpServer};
use nexus::endpoint::{TCPSocket};
use nexus::router::{HttpRouter, FutureResponse};

fn main() {
  let socket = TCPSocket::builder()
    .bind("127.0.0.1:3000")
    .build();

  let http_server = HttpServer::builder()
    .listen(socket)
    .build();

  let channel = Channel::builder()
    .protocol(http_server)
    .build();

  let container = Container::builder()
    .inbound(channel) 
    .build();

  fn index() -> FutureResponse {
    // TODO Implement
    unimplemented!();
  }

  let router = HttpRouter::builder()
    .get("/", index, "index")
    .build();

  let server = AppServer::builder()
    .container(container)
    .router(router)
    // TODO add apps / services
    .build()
    .run();

  match server.results() {
    Ok(_) => println!("Done"),
    Err(e) => println!("Error running server: {:?}", e)
  }
}
