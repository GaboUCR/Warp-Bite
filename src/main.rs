use warp::{Filter, Rejection, Reply};
mod handler;

type WarpResult<T> = std::result::Result<T, Rejection>;

pub async fn ws_handler(ws: warp::ws::Ws) -> WarpResult<impl Reply> {
    // this is where user authentication will happen
    let client = Some(1);
    match client {
        Some(_c) => Ok(ws.on_upgrade(move |socket| handler::client_connection(socket))),
        None => Err(warp::reject::not_found()),
    }
}

#[tokio::main]
async fn main() {
    //enable tokio console
    // console_subscriber::init();

    let ws_route = warp::path("ws").and(warp::ws()).and_then(ws_handler);

    println!("{}", "listening on port 8000");
    warp::serve(ws_route).run(([127, 0, 0, 1], 8000)).await;
}
