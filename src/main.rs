use futures::stream::SplitSink;
use tokio::io::AsyncReadExt;

use tokio::net::TcpStream;
use url::Url;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel};

use futures::{StreamExt, pin_mut, future, SinkExt};

type Tx = UnboundedSender<Message>;
type Rx = UnboundedReceiver<Message>;

#[tokio::main]
async fn main() {
    let url = "ws://localhost:8080/ws";
    let (ws_stream, response) = connect_async(
        Url::parse(url).unwrap()
        ).await.expect("Cannot connect to server");

    let (mut write, read) = ws_stream.split();
    write.send(Message::text("Hello World")).await.unwrap();
    
    println!("Connected to {}", url);

    let (stdin_tx, mut stdin_rx): (Tx, Rx) = unbounded_channel();

    // handle stdin
    tokio::spawn(read_from_stdin(stdin_tx));
    
    // handle message from server
    let ws_stdout = read.for_each(|message| async {
        match message {
            Ok(msg) => {
                println!("Received: {}", msg);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    });
    
    // handle message from stdin
    // let stdin_ws = stdin_rx.map(|x| Ok(x)).forward(write);
    let stdin_ws = stdin_to_ws(&mut stdin_rx, &mut write);
    pin_mut!(ws_stdout, stdin_ws);
    future::select(ws_stdout, stdin_ws).await;
}


// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_from_stdin(tx: Tx) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => {
                break;
            }
            Ok(n) => n,
        };
        // drop old buffer message
        buf.truncate(n);
        tx.send(Message::binary(buf)).unwrap();
    }
}

// forward message from stdin to websocket
async fn stdin_to_ws(rx: &mut Rx, write: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) {
    let buffer = vec![0, 255];
    while let Some(message) = rx.recv().await {
        println!("input: {}", message);
        write.send(message).await.unwrap();
    }
}
