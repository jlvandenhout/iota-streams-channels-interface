use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
use warp::http::StatusCode;

use iota_streams::{
    app::transport::tangle::{
        client::Client,
        PAYLOAD_BYTES,
    },
    app_channels::api::tangle::Author,
};


#[tokio::main]
async fn main() {
    let author = Arc::new(
        Mutex::new(
            Author::new(
                "AUTHORSEED",
                "utf-8",
                PAYLOAD_BYTES,
                true,
                Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com")
            )
        )
    );

    let announce = warp::get()
        .and(with_reference(author.clone()))
        .and_then(send_announce);

    let port = 3030;
    println!("Serving on: http://localhost:{}", port);
    warp::serve(announce)
        .run(([127, 0, 0, 1], port))
        .await;
}


fn with_reference<T: Clone + Send>(
    reference: T
) -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || reference.clone())
}


async fn send_announce(
    author: Arc<Mutex<Author<Client>>>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    let mut author = author.lock().await;

    // BREAKING:
    match author.send_announce().await {
        Ok(address) => {
            Ok(StatusCode::OK)
        },
        Err(_) => {
            Ok(StatusCode::CONFLICT)
        },
    }
}