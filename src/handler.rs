use futures::channel::mpsc;
use futures::{FutureExt, StreamExt};
use std::io::ErrorKind::{Interrupted, WouldBlock};
use std::io::Result;
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};
use warp::ws::{Message, WebSocket};

pub async fn client_connection(ws: WebSocket) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();

    //if connection fails re attempts up to 78 seconds
    let stream = bite_connect().await.unwrap();

    let (byte_rx, byte_tx) = stream.into_split();

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

pub async fn bite_connect() -> Result<TcpStream> {
    let mut connection_attempts = 1;

    loop {
        match TcpStream::connect("127.0.0.1:1984").await {
            Ok(stream) => return Ok(stream),

            Err(e) => {
                if connection_attempts > 12 {
                    return Err(e.into());
                }
            }
        }
        sleep(Duration::new(connection_attempts, 0)).await;
        connection_attempts += 1;
    }
}
