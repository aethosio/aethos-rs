extern crate nexus;

use nexus::{ Container, Channel };
use nexus::protocol::{ Http };

fn main() {
  let channel = Channel::builder()
    .listen("127.0.0.1:3000".parse().unwrap())
    .protocol(Http::new())
    .build();

  let container = Container::builder()
    .inbound(channel) 
    build();
}
