use tokio::net::ToSocketAddrs;
use tokio::runtime::Runtime;

pub use tokio_topics_bridging_with_sync_code::clients::client::Message;

// Established connection with a Redis server.
// structure synchronous interface to mini-redis
pub struct BlockingClient {
    /// The asyncronous `Client` from mini-redis
    inner: tokio_topics_bridging_with_sync_code::clients::Client,
    /// A current_thread runtime for executing operations on the 
    /// asynchronous client in a bloing manner
    rt: Runtime,
    // Runtime fomr Tokio responsible for executing asynchronous operation
}

impl BlockingClient {
    // connect method is instace of BlockingClient T is implement ToSocketAddrs that converted to a socket address
    pub fn connect<T: ToSocketAddrs>(addr: T) -> tokio_topics_bridging_with_sync_code::Result<BlockingCient> {
        let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all() // enable IO and timer drivers on the runtime.
        .build()?;
        // call the asynchronous connect mehtod using the runtime.
        let inner = rt.block_on(tokio_topics_bridging_with_sync_code::clients::Client::connect(addr))?;
        Ok(BlockingCient {inner, rt})
    }

    pub fn get(&mut self, key:&str) -> tokio_topics_bridging_with_sync_code::Result<Option<Bytes>> {
        self.rt.block_on(self.inner.get(key))
    }

    pub fn set(&mut self, key: &str, value: Bytes) -> tokio_topics_bridging_with_sync_code::Result<()> {
        self.rt.block_on(self.inner.set(key, value))
    }

    pub fn set_expires (
        &mut self,
        key: &str,
        value: Bytes,
        expiration: Duration,
    )-> tokio_topics_bridging_with_sync_code::Result<()> {
        self.rt.block_on(self.inner.set_expires(key, value, expiration))
    }
        
    pub fn publish(&mut self, channel: &str, message: Bytes) -> tokio_topics_bridging_with_sync_code::Result<u64> {
        self.rt.block_on(self.inner.publish(channel, message))
    }

    pub fn subscribe(self, channels: Vec<String>) -> tokio_topics_bridging_with_sync_code::Result<BlockingSubscriber>{
        let subscriber = self.rt.block_on(self.inner.subscribe(channel))?;
        Ok(BlockingSubscriber {
            inner: subscriber,
            rt: self.rt
        })
    }

}



pub struct BlockingSubscriber {
    inner: tokio_topics_bridging_with_sync_code::clients::Subscriver,
    rt: Runtime
}



impl BlockingSubscriber {
    pub fn get_subscribed(&self) -> &[String] {
        self.inner.get_subscribed()
    }
    pub fn next_message(&mut self) -> tokio_topics_bridging_with_sync_code::Result<Option<Message>> {
        self.rt.block_on(self.inner.next_message())
    }
    pub fn subscribe(&mut self, channels: &[String]) -> crate::Result<()> {
        self.rt.block_on(self.inner.subscribe(channels))
    }

    pub fn unsubscribe(&mut self, channels: &[String]) -> crate::Result<()> {
        self.rt.block_on(self.inner.unsubscribe(channels))
    }










}













