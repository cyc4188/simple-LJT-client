use std::error::Error;

use tokio;
use tonic::transport::Channel;
use tonic::Request;
use uuid;
use simple_ljt_client::game::{ConnectRequest, game_client::GameClient, StreamRequest, Player, PlayCards, Card};


async fn run_stream(client: &mut GameClient<Channel>, id: String) -> Result<(), Box<dyn Error>>{
    let outbound = async_stream::stream! {
        for i in 0..2 {
            let request = StreamRequest {
                    play_cards: Some(PlayCards {
                        cards: vec![Card{ suit: 1, rank: 1 } ],
                        player: Some(Player {
                            id: id.clone(),
                            name: "test".into(),
                            score: 0,
                            card_num: 0,
                        }),
                    }),
                };
            yield request;
        }
    };

    let response = client.stream(outbound).await?;
    let mut inbound = response.into_inner();
    while let Some(resp) = inbound.message().await? {
        println!("resp = {:?}", resp);
    }

    return Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let id = uuid::Uuid::new_v4().to_string();
    let dest = "http://[::1]:8080";
    let mut client = GameClient::connect(dest).await?;

    let connect_request = Request::new(ConnectRequest {
        id: id.clone(),
        name: "test".into(),
    });
    let response = client.connecting(connect_request).await?;
    println!("RESPONSE={:?}", response);

    run_stream(&mut client, id).await?;

    return Ok(());
}
