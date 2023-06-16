use simple_ljt_client::game::Game;
use simple_ljt_client::proto::{
    self, game_client::GameClient, stream_request, ConnectRequest, StreamRequest, StreamResponse,
};
use simple_ljt_client::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (reponse_sender, response_receiver) = tokio::sync::mpsc::channel::<StreamResponse>(64);

    let (request_sender, request_receiver) = tokio::sync::mpsc::channel::<StreamRequest>(64);

    // uuid
    let id = uuid::Uuid::new_v4().to_string();
    // server dest ip
    let dest = "http://[::1]:8080".to_string();

    // start server
    let server = Server::new(reponse_sender.clone(), id.clone(), dest.clone()).await;
    let handle = tokio::task::spawn(server.start_server(request_receiver));

    // start game loop
    let mut game = Game::new(id.clone(), request_sender, response_receiver);
    game.game_loop();

    handle.await.unwrap();

    return Ok(());
}
