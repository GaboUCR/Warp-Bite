use warp::{Filter, Rejection, Reply};
mod handler;
use thiserror::Error;
type WarpResult<T> = std::result::Result<T, Rejection>;
use std::env;
use std::process::exit;
use warp::http::header::{HeaderMap, HeaderValue};

const API_TOKEN: &str = "6smtr8ke3s7yq63f3zug9z3th";

#[derive(Error, Debug)]
enum ApiErrors {
    #[error("user not authorized")]
    NotAuthorized(String),
}

impl warp::reject::Reject for ApiErrors {}

pub async fn ws_handler(ws: warp::ws::Ws) -> WarpResult<impl Reply> {
    // this is where user authentication will happen
    let client = Some(1);
    match client {
        Some(_c) => Ok(ws.on_upgrade(move |socket| handler::client_connection(socket))),
        None => Err(warp::reject::not_found()),
    }
}

async fn ensure_authentication() -> impl Filter<Extract = (String,), Error = warp::reject::Rejection> + Clone {
    warp::header::optional::<String>("cookie").and_then(
        |cookie_header: Option<String>| async move {
            if let Some(header) = cookie_header {
                let parts: Vec<&str> = header.split("=").collect();
                println!("{:?}", parts);
                //@to-do Fails if we send several parts
                if parts.len() == 2 && parts[0] == "token" && parts[1] == API_TOKEN {
                    return Ok("Existing user".to_string());
                }
            }

            Err(warp::reject::custom(ApiErrors::NotAuthorized (
                "not authorized".to_string(),
            )))
        },
    )
}

#[tokio::main]
async fn main() {
    //enable tokio console
    // console_subscriber::init();
    // let mut headers = HeaderMap::new();
    // headers.insert("set-cookie", HeaderValue::from_static("token=6smtr8ke3s7yq63f3zug9z3th; path=/"));
    
    // not compatible with warp::server::run
    // let server_uri = match env::var("SERVER") {
    //     Ok(v) => v,
    //     Err(_) => {
    //         println!("Environmental variable SERVER is missing!");
    //         println!("That's the uri where the server runs");
    //         println!("BASH i.e: export SERVER=0.0.0.0:8000");
    //         exit(1);
    //     }
    // };

    let register = warp::path("static").and(warp::fs::dir("./static")); //.with(warp::reply::with::headers(headers));

    let ws_route = warp::path("ws").and(warp::ws()).and_then(ws_handler);
        // .and(ensure_authentication().await)

    let routes = register.or(ws_route);
    println!("{}", "listening on port 8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    // warp::serve(routes).run(server_uri).await;
}
