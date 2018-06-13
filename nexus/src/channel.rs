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
      // TODO implement
      let ChannelBuilder(mut inner) = self;
      inner.listen(address);
      ChannelBuilder(inner)
    }
}