use std::net::SocketAddr;
use std::sync::Arc;

use solana_client::connection_cache::ConnectionCache;
use solana_client::thin_client::ThinClient;
// use solana_client::tpu_client::TpuSenderError;

// type Result<T> = std::result::Result<T, TpuSenderError>;

pub struct LightRpc {
    pub connection_cache: Arc<ConnectionCache>,
    pub thin_client: ThinClient,
}

impl LightRpc {
    pub fn new(rpc_addr: SocketAddr, tpu_addr: SocketAddr, connection_pool_size: usize) -> Self {
        let connection_cache = Arc::new(ConnectionCache::new(connection_pool_size));
        let thin_client = ThinClient::new(rpc_addr, tpu_addr, connection_cache.clone());

        Self {
            connection_cache,
            thin_client,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::LightRpc;

    const RPC_ADDR: &str = "127.0.0.1:8899";
    const TPU_ADDR: &str = "127.0.0.1:1027";
    const CONNECTION_POOL_SIZE: usize = 1;

    #[test]
    fn initialize_light_rpc() {
        let _light_rpc = LightRpc::new(
            RPC_ADDR.parse().unwrap(),
            TPU_ADDR.parse().unwrap(),
            CONNECTION_POOL_SIZE,
        );
    }
}
