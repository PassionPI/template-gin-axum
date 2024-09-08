use redis::Client;

#[derive(Clone)]
pub struct Rd {
    pub client: Client,
}

impl Rd {
    pub async fn new(uri: &str) -> Self {
        let client = redis::Client::open(uri).expect("Failed to open redis client");
        println!("Connected to redis.");
        Self { client }
    }

    pub async fn conn(&self) -> anyhow::Result<redis::aio::MultiplexedConnection> {
        Ok(self.client.get_multiplexed_async_connection().await?)
    }
}
