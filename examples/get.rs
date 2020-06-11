use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use futures_util::future;
use redis_async::{client, resp_array};

#[tokio::main]
async fn main() {
    let test_data_size = 10;
    let test_data: Vec<_> = (0..test_data_size).map(|x| (x, x.to_string())).collect();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6379);

    let connection = client::paired_connect(&addr)
        .await
        .expect("Cannot open connection");

    let futures = test_data.into_iter().map(|data| {
        let connection_inner = connection.clone();
        let incr_f = connection.send(resp_array!["INCR", "realistic_test_ctr"]);
        async move {
            let ctr: String = incr_f.await.expect("Cannot increment");

            let key = format!("rt_{}", ctr);
            let d_val = data.0.to_string();
            connection_inner.send_and_forget(resp_array!["SET", &key, d_val]);
            connection_inner
                .send(resp_array!["SET", data.1, key])
                .await
                .expect("Cannot set")
        }
    });
    let result: Vec<String> = future::join_all(futures).await;
    println!("RESULT: {:?}", result);
}
