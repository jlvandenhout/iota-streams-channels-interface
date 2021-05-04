use futures::{SinkExt, StreamExt};
use warp::{Buf, Filter, Rejection, Reply};
use warp::multipart::FormData;
use std::convert::From;


#[tokio::main]
async fn main() {
    let data = warp::path("data").map
        .and(warp::ws())
        .map(
            |ws: warp::ws::Ws| {
                ws.on_upgrade(move |socket| handle_socket(socket))
            }
        );

    let index = warp::path::end()
        .and(warp::fs::file("client/index.html"));

    let client = warp::get()
        .and(warp::fs::dir("client"));

    let form = warp::path("form")
        .and(warp::multipart::form())
        .and_then(handle_form);

    let routes = index
        .or(client)
        .or(data)
        .or(form);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}


async fn handle_form(mut form: FormData) -> Result<impl Reply, Rejection> {
    while let Some(result) = form.next().await {
        if let Ok(mut part) = result {
            let mut bytes : Vec<u8> = Vec::new();

            while let Some(result) = part.data().await {
                if let Ok(buffer) = result {
                    bytes.extend_from_slice(buffer.chunk());
                } else {
                    return Err(warp::reject());
                }
            }

            let name = String::from(part.name());
            let value = match String::from_utf8(bytes) {
                Ok(value) => value,
                Err(_) => return Err(warp::reject()),
            };

            println!("{}: {}", name, value);
        } else {
            return Err(warp::reject());
        }
    }

    Ok(warp::reply())
}


async fn handle_socket(mut socket: warp::ws::WebSocket) {
    while let Some(input) = socket.next().await {
        match input {
            Ok(message) => {
                if message.is_close() {
                    break;
                }

                if message.is_text() {
                    if let Err(error) = socket.send(message).await {
                        println!("{}", error);
                    }
                }
            },
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}