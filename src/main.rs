use simple_ljt_client::proto::{
    self, game_client::GameClient, stream_request, ConnectRequest, StreamRequest, StreamResponse,
};
use simple_ljt_client::server::Server;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::tungstenite::http::request;
use tonic::transport::Channel;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (reponse_sender, response_receiver) = tokio::sync::mpsc::channel::<StreamResponse>(64);

    let (request_sender, request_receiver) = tokio::sync::mpsc::channel::<StreamRequest>(64);

    let id = uuid::Uuid::new_v4().to_string();
    let dest = "http://[::1]:8080".to_string();

    let server = Server::new(reponse_sender.clone(), id.clone(), dest.clone()).await;

    tokio::task::spawn(server.start_server(request_receiver));

    // let handle = tokio::task::spawn(run_stream(&mut client, id, rx));

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
