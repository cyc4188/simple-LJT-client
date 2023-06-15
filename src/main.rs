use simple_ljt_client::card::{show_cards, Card};

use simple_ljt_client::proto::{
    self, game_client::GameClient, stream_request, ConnectRequest, StreamRequest,
};
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc::{Receiver, Sender};
use tonic::transport::Channel;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = uuid::Uuid::new_v4().to_string();
    let dest = "http://[::1]:8080";
    let mut client = GameClient::connect(dest).await?;

    let connect_request = Request::new(ConnectRequest {
        id: id.clone(),
        name: "test".into(),
    });
    let response = client.connecting(connect_request).await?;
    println!("RESPONSE={:?}", response);

    // stream
    let (mut tx, rx) = tokio::sync::mpsc::channel::<StreamRequest>(16);
    run_stream(&mut client, id, rx).await;

    // let handle = tokio::task::spawn(run_stream(&mut client, id, rx));

    // sender
    keyboard(tx.clone()).await;

    return Ok(());
}

async fn keyboard(tx: Sender<StreamRequest>) {}

async fn run_stream(client: &mut GameClient<Channel>, id: String, mut rx: Receiver<StreamRequest>) {
    let outbound = async_stream::stream! {
        for i in 0..1 {
            let request = StreamRequest {
                request:
                    Some(
                        stream_request::Request::PlayCards(
                            proto::PlayCards {
                                player: Some(proto::Player {
                                    id: id.clone(),
                                    name: "test".into(),
                                    score: 0,
                                    card_num: 0,
                                    index: 0,
                                }),
                                cards: vec![proto::Card{ suit: 1, rank: 1 } ],
                            }
                            )
                        )
            };
        }

        while let Some(request) = rx.recv().await {
            yield request;
        }
    };

    let response = client.stream(outbound).await.unwrap();
    let mut inbound = response.into_inner();
    while let Some(resp) = inbound.message().await.unwrap() {
        println!("resp = {:?}", resp);
    }
}
