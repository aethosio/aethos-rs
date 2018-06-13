use super::channel::Channel;
use super::protocol::Protocol;

/**
 * Public types
 */
pub struct Container(inner::Container);
pub struct ContainerBuilder(inner::ContainerBuilder);

/**
 * Private implementation
 */
mod inner {
/**
 * Container
 */
pub struct Container {
    x: i32
}

impl Container {
    pub fn new() -> Container {
        Container{ x: 0 }
    }    
}

pub struct ContainerBuilder {
    channels: Vec<super::super::channel::Channel>
}
impl ContainerBuilder {    
    pub fn new() -> ContainerBuilder {
        ContainerBuilder{channels: Vec::new()}
    }
    pub fn listen(&mut self, endpoint: super::Channel) {
        self.channels.push(endpoint);
    }
    pub fn protocol(&mut self, protocol: super::Protocol) {
        // TODO Implement
    }
}
}

/**
 * Public proxies for newtypes 
 */
impl Container {
    pub fn builder() -> ContainerBuilder {
        ContainerBuilder(inner::ContainerBuilder::new())
    }

    pub fn new() -> Container {
        Container(inner::Container::new())
    }
}

impl ContainerBuilder {
    pub fn listen(self, channel: Channel) -> ContainerBuilder {
        let ContainerBuilder(mut inner) = self;
        inner.listen(channel);
        ContainerBuilder(inner)
    }
    pub fn protocol(self, protocol: Protocol) -> ContainerBuilder {
        let ContainerBuilder(mut inner) = self;
        inner.protocol(protocol);
        ContainerBuilder(inner)
    }
}
