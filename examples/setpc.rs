// This uses paired_connect instead of connect
// which is shown in the example set.rs

use futures::{sink::SinkExt};
use redis_async::{client, resp_array};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6379);
    assert_eq!("127.0.0.1:6379".parse(), Ok(addr));

    let connection = client::paired_connect(&addr)
        .await
        .expect("Cannot open connection");
}