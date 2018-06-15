extern crate hyper;
extern crate route_recognizer as recognizer;
extern crate futures;
extern crate http;

pub mod channel;
pub mod container;
pub mod endpoint;
pub mod protocol;
pub mod router;
pub mod server;

pub use channel::Channel;
pub use container::Container;
pub use server::AppServer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
