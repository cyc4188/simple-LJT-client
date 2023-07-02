use std::cell::RefCell;
use std::io::stdout;
use std::rc::Rc;

use simple_ljt_client::game::Game;
use simple_ljt_client::player::Client;
use simple_ljt_client::proto::{
    self, game_client::GameClient, stream_request, ConnectRequest, StreamRequest, StreamResponse,
};
use simple_ljt_client::server::Server;
use simple_ljt_client::ui::loginui;
use tui::backend::CrosstermBackend;
use tui::Terminal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (reponse_sender, response_receiver) = tokio::sync::mpsc::channel::<StreamResponse>(64);
    let (request_sender, request_receiver) = tokio::sync::mpsc::channel::<StreamRequest>(64);

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    let terminal_rc = Rc::new(RefCell::new(terminal));

    // uuid
    let id = uuid::Uuid::new_v4().to_string();
    // name
    let name = if let Some(inner) = loginui::LoginUI::new(terminal_rc.clone()).screen() {
        inner
    } else {
        return Ok(());
    };

    let client = Rc::new(RefCell::new(Client::new(id.clone(), name.clone())));

    // server dest ip
    let dest = "http://[::1]:8080".to_string();

    // start server
    let server = Server::new(
        reponse_sender.clone(),
        name.clone(),
        id.clone(),
        dest.clone(),
    )
    .await;
    tokio::task::spawn(server.start_server(request_receiver));

    // start game loop
    let mut game = Game::new(client.clone(), request_sender, response_receiver);
    game.game_loop(terminal_rc).await;

    // handle.await.unwrap();

    return Ok(());
}
