
use super::protocol::{ Protocol };

/**
 * Public types
 */
pub struct Channel(inner::Channel);
pub struct ChannelBuilder(inner::ChannelBuilder);


/**
 * Private implementation
 */
mod inner {
pub struct Channel {
}
impl Channel {
}
pub struct ChannelBuilder {
}
impl ChannelBuilder {
  pub fn new() -> ChannelBuilder {
    ChannelBuilder{}
  }
  pub fn listen(&mut self, address : String) {
    // TODO Implement
    unimplemented!();
  }
  pub fn protocol(&mut self, protocol: &super::Protocol)   {
    // TODO Implement
    unimplemented!();
  }
}
}

/**
 * Public proxies for newtypes 
 */
impl Channel {
    pub fn builder() -> ChannelBuilder {
        ChannelBuilder(inner::ChannelBuilder::new())
    }
}

impl ChannelBuilder {
    pub fn listen(self, address : String) -> ChannelBuilder {
      let ChannelBuilder(mut inner) = self;
      inner.listen(address);
      ChannelBuilder(inner)
    }

    pub fn protocol(self, protocol: &Protocol) -> ChannelBuilder {
      let ChannelBuilder(mut inner) = self;
      inner.protocol(protocol);
      ChannelBuilder(inner)
    }

    pub fn build(self) -> Channel {
      unimplemented!();
    }
}
