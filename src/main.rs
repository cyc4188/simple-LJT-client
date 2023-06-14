use std::error::Error;

use simple_ljt_client::game::{
    self, game_client::GameClient, stream_request, stream_response, ConnectRequest, StreamRequest,
};
use simple_ljt_client::Card;
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
    // run_stream(&mut client, id, rx).await?;
    tokio::task::spawn(run_stream(client, id, rx));

    // sender
    keyboard(tx.clone()).await;

    return Ok(());
}

async fn keyboard(tx: Sender<StreamRequest>) {
    let stdin = tokio::io::stdin();
    let mut lines = tokio::io::BufReader::new(stdin).lines();

    while let Some(line) = lines.next_line().await.unwrap() {
        println!("you entered {}", line);
    }
}

async fn run_stream(mut client: GameClient<Channel>, id: String, mut rx: Receiver<StreamRequest>) {
    let outbound = async_stream::stream! {
        for i in 0..1 {
            let request = StreamRequest {
                request:
                    Some(
                        stream_request::Request::PlayCards(
                            game::PlayCards {
                                player: Some(game::Player {
                                    id: id.clone(),
                                    name: "test".into(),
                                    score: 0,
                                    card_num: 0,
                                    index: 0,
                                }),
                                cards: vec![game::Card{ suit: 1, rank: 1 } ],
                            }
                            )
                        )
            };
            yield request;
        }

        while let Some(request) = rx.recv().await {
            yield request;
        }
    };

    // get response from game_stream
    let response = client.stream(outbound).await.unwrap();
    let mut inbound = response.into_inner();
    while let Some(resp) = inbound.message().await.unwrap() {
        match resp.response {
            Some(stream_response::Response::Continue(cont)) => {
                println!(
                    "continue = {}",
                    cont.cards
                        .iter()
                        .map(|card| Card::from(card).to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
            }
            Some(stream_response::Response::Fail(fail)) => {
                println!("fail = {:?}", fail);
            }
            Some(stream_response::Response::End(end)) => {
                println!("end = {:?}", end);
            }
            _ => {}
        }
    }
}
