use futures::future::ok;
use std::error::Error;
use std::io::ErrorKind::{BrokenPipe, Interrupted, WouldBlock};
use tokio::time::{sleep, Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use futures::channel::mpsc;
use futures::{FutureExt, StreamExt};
use warp::ws::{Message, WebSocket};
use warp::{Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

pub async fn client_connection(ws: WebSocket) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();

    let mut stream = TcpStream::connect("127.0.0.1:1984").await.expect("bite connection refused :<");
    let (mut byte_rx, mut byte_tx) = stream.into_split();

    let (client_sender, client_rcv) = mpsc::unbounded();

    // check for new messages from Bite
    let bite_read_handler = tokio::task::spawn(async move {
        loop {
            let mut b2 = [0; 100];
            let _success = byte_rx.readable().await;

            //Saves bite's response on b2 buffer
            let p = byte_rx.try_read(&mut b2);

            match p {
                Ok(i) => {
                    // Reading 0 bytes means the other side has closed the
                    // connection or is done writing, then so are we.
                    if i == 0 {
                        break;
                    }
                    let s = String::from_utf8_lossy(&b2);
                    let _success = client_sender.unbounded_send(Ok(Message::text(s)));
                }
                Err(ref err) if err.kind() == WouldBlock => continue,

                Err(ref err) if err.kind() == Interrupted => continue,

                // Other errors we'll consider fatal.
                Err(_err) => break,
            };
        }
    });

    let bite_proxy = tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        //this task is ended when bite_read_handler ends
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));
    // Check for new messages from the Client
    let client_handler = tokio::task::spawn(async move {
        while let Some(result) = client_ws_rcv.next().await {
            let msg = match result {
                Ok(msg) => msg,
                //should this close the connection??
                Err(e) => {
                    eprintln!("error receiving ws message {}", e);
                    break;
                }
            };
            //write to bite
            let _success = byte_tx.writable().await;

            let n_bytes = byte_tx.try_write(&msg.into_bytes());

            match n_bytes {
                Ok(_i) => continue,
                Err(e) => println!("Unable to write {}", e),
            };
        }
    });

    println!("{}", "connected");

    //If any task closes every other task needs to close
    tokio::select! {
        _ = client_handler => {
            println!("The websocket was disconnected");
        }
        _ = bite_proxy => {
            println!("bite proxy got disconnected");
        }
        _ = bite_read_handler => {
            println!("Bite got disconnected");
        }
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
    let ws_route = warp::path("ws").and(warp::ws()).and_then(ws_handler);

    println!("{}", "listening on port 8000");
    warp::serve(ws_route).run(([127, 0, 0, 1], 8000)).await;
}
