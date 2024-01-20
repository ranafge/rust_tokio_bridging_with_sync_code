use tokio::net::ToSocketAddrs;
use tokio::runtime::Runtime;

pub use tokio_topics_bridging_with_sync_code::clients::client::Message;

// Established connection with a Redis server.
// structure synchronous interface to mini-redis
pub struct BlockingCient {
    /// The asyncronous `Client` from mini-redis
    inner: tokio_topics_bridging_with_sync_code::clients::Client,
    /// A current_thread runtime for executing operations on the 
    /// asynchronous client in a bloing manner
    rt: Runtime,
    // Runtime fomr Tokio responsible for executing asynchronous operation
}

impl BlockingCient {
    // connect method is instace of BlockingClient T is implement ToSocketAddrs that converted to a socket address
    pub fn connect<T: ToSocketAddrs>(addr: T) -> tokio_topics_bridging_with_sync_code::Result<BlockingCient> {
        let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all() // enable IO and timer drivers on the runtime.
        .build()?;
        // call the asynchronous connect mehtod using the runtime.
        let inner = rt.block_on(tokio_topics_bridging_with_sync_code::clients::Client::connect(addr))?;
        Ok(BlockingCient {inner, rt})
    }
}