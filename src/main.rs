
use futures::{SinkExt, StreamExt};
use warp::{Filter, Rejection, Reply};


async fn form_handler(mut form_data: warp::multipart::FormData) -> Result<impl Reply, Rejection> {
    while let Some(part) = form_data.next().await {
        match part {
            Ok(message) => {
                println!("{}", message.name());
                let stream = message.stream();
                while let Some(s) = stream.next().await {
                    match s {
                        Ok(data) => {
                            let string = String::from_utf8(data);
                        }
                    }
                }
            },
            Err(_) => {
                return Err(warp::reject())
            },
        }
    }
    Ok(warp::reply())
}

#[tokio::main]
async fn main() {
    let data = warp::path("data")
        .and(warp::ws())
        .map(
            |ws: warp::ws::Ws| {
                ws.on_upgrade(move |socket| handle(socket))
            }
        );

    let index = warp::path::end()
        .and(warp::fs::file("client/index.html"));

    let client = warp::get()
        .and(warp::fs::dir("client"));

    let form = warp::path("form")
        .and(warp::multipart::form())
        .and_then(form_handler);

    let routes = index
        .or(client)
        .or(data)
        .or(form);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle(mut socket: warp::ws::WebSocket) {
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