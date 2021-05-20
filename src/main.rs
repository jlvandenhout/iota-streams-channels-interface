use std::convert::Infallible;

use futures::SinkExt;
use serde::Deserialize;
use tokio::sync::{mpsc, watch};
use warp::{
    http::StatusCode,
    Filter,
};
use warp::ws::Message;

use iota_streams::app_channels::api::tangle::{
    Author,
    UnwrappedMessage,
};
use iota_streams::app::transport::tangle::{
    PAYLOAD_BYTES,
    client::Client,
};

#[derive(Deserialize)]
enum Input {
    Announce,
}


#[tokio::main]
async fn main() {
    let (input_sender, mut input_receiver) = mpsc::channel::<Input>(8);
    let (output_sender, output_receiver) = watch::channel(String::new());

    let mut author = Author::new(
        "AUTHORSEED",
        "utf-8",
        PAYLOAD_BYTES,
        true,
        Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com")
    );

    let input = warp::post()
        .and(warp::body::json())
        .and(warp::any().map(move || input_sender.clone()))
        .and_then(handle_input);

    let output = warp::ws()
        .and(warp::any().map(move || output_receiver.clone()))
        .map(handle_socket_upgrade);

    loop {
        tokio::select!{
            messages = receive_messages(&mut author, 1) => {
                for message in messages {
                    output_sender.send(message.link.to_string()).unwrap();
                }
            },
            option = input_receiver.recv() => {
                if let Some(input) = option {
                    match input {
                        Input::Announce => {
                            println!("Announce");
                            let link = author.send_announce().await.unwrap();
                            output_sender.send(link.to_string()).unwrap();
                        }
                    }
                } else {
                    break;
                }
            }
        }
    };

    warp::serve(input.or(output)).run(([127, 0, 0, 1], 3030)).await;
}


async fn receive_messages(author: &mut Author<Client>, interval: u64) -> Vec<UnwrappedMessage> {
    let mut messages = Vec::new();
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval));
    while messages.is_empty() {
        interval.tick().await;
        messages = author.fetch_next_msgs().await;
    }
    messages
}


async fn handle_input(
    input: Input,
    input_sender: mpsc::Sender<Input>,
) -> Result<impl warp::Reply, Infallible> {
    if input_sender.send(input).await.is_ok() {
        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::BAD_REQUEST)
    }
}


fn handle_socket_upgrade(
    output_socket_state: warp::ws::Ws,
    output_receiver: watch::Receiver<String>
) -> impl warp::Reply {
    output_socket_state.on_upgrade(
        |output_socket| handle_output(output_socket, output_receiver)
    )
}


async fn handle_output(
    mut output_socket: warp::ws::WebSocket,
    mut output_receiver: watch::Receiver<String>,
) {
    while output_receiver.changed().await.is_ok() {
        let output = output_receiver.borrow().clone();
        output_socket.send(Message::text(output)).await.unwrap();
    }
}