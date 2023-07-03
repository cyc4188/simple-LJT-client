use crate::proto;
use tokio::sync::mpsc::{Receiver, Sender};
use tonic::transport::Channel;

use crate::proto::{
    game_client::GameClient, stream_request, ConnectRequest, StreamRequest, StreamResponse,
};
use tonic::Request;

pub struct Server {
    pub response_sender: Sender<StreamResponse>,
    pub game_client: GameClient<Channel>,
    pub id: String,
    pub name: String,
}

impl Server {
    pub async fn new(
        response_sender: Sender<StreamResponse>,
        name: String,
        id: String,
        dest: String,
    ) -> Self {
        let mut game_client = GameClient::connect(dest)
            .await
            .expect("cannot connect to the server");

        // 首先发送一个 ConnectRequest
        let connect_request = Request::new(ConnectRequest {
            id: id.clone(),
            name: name.clone(),
        });
        let response = game_client
            .connecting(connect_request)
            .await
            .expect("cannot connect to the server");
        // println!("RESPONSE={:?}", response);

        Self {
            response_sender,
            id,
            game_client,
            name,
        }
    }

    pub async fn start_server(mut self, mut rx: Receiver<StreamRequest>) {
        let id = self.id.clone();
        let name = self.name.clone();
        let outbound = async_stream::stream! {
            for i in 0..1 {
                let request = StreamRequest {
                    request:
                        Some(
                            stream_request::Request::PlayCards(
                                proto::PlayCards {
                                    player: Some(proto::Player {
                                        id: id.clone(),
                                        name: name.clone(),
                                        score: 0,
                                        card_num: 0,
                                        index: 0,
                                    }),
                                    cards: vec![proto::Card{ suit: 1, rank: 1 } ],
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

        let response = self.game_client.stream(outbound).await.unwrap();
        let mut inbound = response.into_inner();
        while let Some(resp) = inbound.message().await.unwrap_or(None) {
            // println!("resp = {:?}", resp);
            self.response_sender
                .send(resp)
                .await
                .expect("cannot send response to game");
        }
        println!("server disconnect");
    }
}
