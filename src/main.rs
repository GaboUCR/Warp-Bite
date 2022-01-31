use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};
use std::error::Error;
use tokio::time::{sleep, Duration};
// use tokio::sync::mpsc;
// use warp::{http::StatusCode, reply::json, ws::Message, Reply};
use warp::{Reply, Filter, Rejection};
use futures::{FutureExt, StreamExt};
use warp::ws::{Message, WebSocket};
use futures::channel::mpsc;

type Result<T> = std::result::Result<T, Rejection>;

pub async fn client_connection(ws: WebSocket) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded();

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    println!("{}", "connected");

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message {}", e);
                break;
            }
        };
        println!("{:?}", msg);
        client_sender.unbounded_send(Ok(Message::text("patito")));
        // client_msg(&id, msg, &clients).await;
    }

    //disconnects user 
    println!("{}", "disconnected");
}

pub async fn ws_handler(ws: warp::ws::Ws) -> Result<impl Reply> {
    // this is where user authentication will happen
    let client = Some(1);
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| client_connection(socket))),
        None => Err(warp::reject::not_found()),
    }
}
#[tokio::main]
async fn main() {
    
    let ws_route = warp::path("ws")
    .and(warp::ws())
    .and_then(ws_handler);
    
    println!("{}", "listening on port 8000");
    warp::serve(ws_route).run(([127, 0, 0, 1], 8000)).await;

}

// #[tokio::main]
// async fn main() -> Result <(), Box<dyn Error>> {
//     let mut stream= TcpStream::connect("127.0.0.1:1984").await?;
//     let mut b2 = [0; 10];
//     let (mut rx, tx) = stream.split();
//     let n = tx.try_write(b"g perro").unwrap();
//     let a = rx.peek(&mut b2).await?;
//     let p = rx.read(&mut b2[..n]).await?;
//     println!("{:?}", b2);
    
//     sleep(Duration::from_millis(5000)).await;
//     Ok(())
// }
