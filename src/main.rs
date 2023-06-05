use tokio;
use uuid;
use simple_ljt_client::game::{ConnectRequest, game_client::GameClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let dest = "http://[::1]:8080";
    let mut client = GameClient::connect(dest).await?;

    let connect_request = tonic::Request::new(ConnectRequest {
        id: uuid::Uuid::new_v4().to_string(),
        name: "test".into(),
    });
    let response = client.connecting(connect_request).await?;
    println!("RESPONSE={:?}", response);

    return Ok(());
}
