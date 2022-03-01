use futures::channel::mpsc;
use futures::{FutureExt, StreamExt};
use std::io::ErrorKind::{Interrupted, WouldBlock};
use std::env;
use std::process::exit;
use std::io::Result;
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};
use warp::ws::{Message, WebSocket};

pub async fn client_connection(ws: WebSocket) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();

    let bite_uri  = match env::var("BITE") {
        Ok(v) => v,
        Err(_) => {
            println!("Environmental variable BITE is missing!");
            println!("The URI where Bite is gonna receive connections.");
            println!("BASH i.e: export BITE=0.0.0.0:1984");
            exit(1);
        }
    };

    //if connection fails re attempts up to 78 seconds
    let stream = tcp_connect(&bite_uri).await.unwrap();

    let (byte_rx, byte_tx) = stream.into_split();
    //@todo use tx rx
    let (client_tx, client_rx) = mpsc::unbounded();

    // check for new messages from Bite
    let bite_read_handler = tokio::task::spawn(async move {
        let mut not_done = true;

        while not_done {
            let mut buffer = vec![0; 1024];
            let mut bytes_read = 0;

            let _success = byte_rx.readable().await;

            loop {
                let p = byte_rx.try_read(&mut buffer);

                match p {
                    Ok(i) => {
                        // Reading 0 bytes means the other side has closed the
                        // connection or is done writing, then so are we.
                        if i == 0 {
                            not_done = false;
                            break;
                        }
                        bytes_read += i;

                        if bytes_read == buffer.len() {
                            buffer.resize(buffer.len() + 1024, 0);
                        }

                        let msg = String::from_utf8_lossy(&buffer);
                        let _success = client_tx.unbounded_send(Ok(Message::text(msg)));
                    }
                    Err(ref err) if err.kind() == WouldBlock => break,

                    Err(ref err) if err.kind() == Interrupted => continue,

                    // Other errors we'll consider fatal.
                    Err(_err) => {
                        not_done = false;
                        break;
                    }
                };
            }
        }
    });

    let bite_proxy = tokio::task::spawn(client_rx.forward(client_ws_sender).map(|result| {
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

                Err(ref err) if err.kind() == WouldBlock => continue,

                Err(ref err) if err.kind() == Interrupted => continue,

                // Other errors we'll consider fatal.
                Err(_e) => break,
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

pub async fn tcp_connect(tcp_uri:&str) -> Result<TcpStream> {
    let mut connection_attempts = 1;

    loop {
        match TcpStream::connect(tcp_uri).await {
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
