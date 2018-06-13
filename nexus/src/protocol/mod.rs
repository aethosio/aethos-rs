mod http;

pub use self::http::*;

/**
 * Public types
 */
pub struct Protocol(inner::Protocol);

/**
 * Private implementation
 */
mod inner {
pub struct Protocol {}
impl Protocol {}
}

/**
 * Public proxies for newtypes 
 */
impl Protocol {

}
