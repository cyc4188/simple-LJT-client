use crate::proto;
use tokio::sync::mpsc::{Receiver, Sender};
use tonic::transport::Channel;

use crate::proto::{game_client::GameClient, stream_request, StreamRequest, StreamResponse};

pub struct Server {
    pub response_sender: Sender<StreamResponse>,
    pub game_client: GameClient<Channel>,
    pub id: String,
}

impl Server {
    pub async fn new(response_sender: Sender<StreamResponse>, id: String, dest: String) -> Self {
        let game_client = GameClient::connect(dest)
            .await
            .expect("cannot connect to the server");
        Self {
            response_sender,
            id,
            game_client,
        }
    }

    pub async fn start_server(&mut self, mut rx: Receiver<StreamRequest>) {
        let id = self.id.clone();
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

        let response = self.game_client.stream(outbound).await.unwrap();
        let mut inbound = response.into_inner();
        while let Some(resp) = inbound.message().await.unwrap() {
            println!("resp = {:?}", resp);
        }
    }
}
