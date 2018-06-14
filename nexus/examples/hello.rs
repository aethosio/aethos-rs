extern crate nexus;

use nexus::{ Container, Channel, AppServer };
use nexus::protocol::{ HttpServer };

fn main() {
  let socket = TcpSocket::builder()
    .bind("127.0.0.1:3000".parse().unwrap())
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

  let server = AppServer::builder()
    .container(container)
    // TODO add apps / services
    .build()
    .run();
}
