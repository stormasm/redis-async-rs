use std::env;
use futures_util::future;
use redis_async::{client, resp_array};

#[tokio::main]
async fn main() {
    // Create some completely arbitrary "test data"
    let test_data_size = 10;
    let test_data: Vec<_> = (0..test_data_size).map(|x| (x, x.to_string())).collect();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6379".to_string())
        .parse()
        .unwrap();

    let connection = client::paired_connect(&addr)
        .await
        .expect("Cannot open connection");

    let futures = test_data.into_iter().map(|data| {
        let connection_inner = connection.clone();
//      let incr_f = connection.send(resp_array!["INCR", "realistic_test_ctr"]);
        async move {
//            let ctr: String = incr_f.await.expect("Cannot increment");

            connection_inner.send_and_forget(resp_array!["SET", "or", "corvallis"]);
            connection_inner
                .send(resp_array!["SET", "nm", "raton 1"])
                .await
                .expect("Cannot set")
        }
    });
    let result: Vec<String> = future::join_all(futures).await;
    println!("RESULT: {:?}", result);
    assert_eq!(result.len(), test_data_size);
}
