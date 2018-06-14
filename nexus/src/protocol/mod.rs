pub mod http_server;

pub use self::http_server::*;

/**
 * Public types
 */
pub trait Protocol {

}

/**
 * Private implementation
 */
mod inner {
}

/**
 * Public proxies for newtypes 
 */
struct undef {}
