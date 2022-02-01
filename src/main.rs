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

pub async fn client_connection(ws: WebSocket)  {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    
    let mut stream = TcpStream::connect("127.0.0.1:1984").await.unwrap();
    let (mut byte_rx, mut byte_tx) = stream.into_split();
    
    let (client_sender, client_rcv) = mpsc::unbounded();
    
    // check for new messages from Bite
    tokio::task::spawn(async move { 
        loop {
            let mut b2 = [0; 100];
            byte_rx.readable().await;

            //Saves bite's response on b2 buffer
            let p = byte_rx.try_read(&mut b2);
            
            match p {
                Ok(_i) => {
                    let s = String::from_utf8_lossy(&b2);
                    client_sender.unbounded_send(Ok(Message::text(s)));
                },
                Err(e) => continue,
            };
    
        }

    });

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    println!("{}", "connected");
    // Check for new messages from the Client
    while let Some(result) = client_ws_rcv.next().await {
        let b2 = [0; 100];
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message {}", e);
                break;
            }
        };
        //write to bite 
        byte_tx.writable().await;
        let n = byte_tx.try_write(&msg.into_bytes()).unwrap();
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

