use http::Request;
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() {
    let socket_url = "ws://example.com/socket";
    let socket_request = Request::builder()
        .uri(socket_url)
        .header("X-Auth-Token", "hi_there")
        .body(())
        .unwrap();
    match connect_async(socket_request).await {
        Ok((_, _)) => {
            println!("yay!");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    };
}
