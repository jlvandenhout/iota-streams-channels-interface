use std::sync::Arc;

use tokio::sync::Mutex;
use warp::{
    http::StatusCode,
    Filter,
};

use iota_streams::app_channels::api::tangle::Author;
use iota_streams::app::transport::tangle::{
    PAYLOAD_BYTES,
    client::Client,
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

    let route = warp::get()
        .and_then(
            || async {
                let mut author = author.lock().await;

                // BREAKING: Why does returned Future not implement Send?
                if author.send_announce().await.is_ok() {
                    Ok(StatusCode::OK)
                } else {
                    Ok(StatusCode::CONFLICT)
                }
            }
        );

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}