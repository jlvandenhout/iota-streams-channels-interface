use std::{
    convert::Infallible,
    sync::Arc,
};

use tokio::sync::Mutex;
use warp::Filter;

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
        .and(with_reference(author.clone()))
        .and_then(send_announce);

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}


fn with_reference<T: Clone + Send>(
    reference: T
) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    warp::any().map(move || reference.clone())
}


async fn send_announce(
    author: Arc<Mutex<Author<Client>>>
) -> Result<impl warp::Reply, Infallible> {
    let mut author = author.lock().await;

    if let Ok(address) = author.send_announce().await {
        Ok(address.to_string())
    } else {
        Ok(String::from("Failed to send announcement"))
    }
}
