use futures::{sink::SinkExt};
use redis_async::{client, resp_array};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6379);
    assert_eq!("127.0.0.1:6379".parse(), Ok(socket));
    // let addr1 = socket;

    let mut connection = client::connect(&socket)
        .await
        .expect("Cannot connect to Redis");
    connection
        .send(resp_array!["SET", "nm", "raton 1"])
        .await
        .expect("Cannot send MONITOR command");
}
