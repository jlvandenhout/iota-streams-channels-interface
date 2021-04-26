
use futures::{SinkExt, StreamExt};
use warp::{
    Filter,
    ws::{
        Ws,
        WebSocket,
    }
};

#[tokio::main]
async fn main() {
    let data = warp::path("data")
        .and(warp::ws())
        .map(|ws: Ws| {
            ws.on_upgrade(move |socket| handle(socket))
        });

    let index = warp::path::end()
        .and(warp::fs::file("static/index.html"));

    let dir = warp::get()
        .and(warp::fs::dir("static"));

    let routes = index
        .or(dir)
        .or(data);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle(mut socket: WebSocket) {
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