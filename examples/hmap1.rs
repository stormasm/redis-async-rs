use std::env;
use futures::{sink::SinkExt, stream::StreamExt};
use redis_async::{client, resp_array};

#[tokio::main]
async fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6379".to_string())
        .parse()
        .expect("Cannot parse Redis connection string");

    let mut connection = client::connect(&addr)
        .await
        .expect("Cannot connect to Redis");
    connection
        .send(resp_array!["SET", "nm", "raton 1"])
        .await
        .expect("Cannot send MONITOR command");
}
