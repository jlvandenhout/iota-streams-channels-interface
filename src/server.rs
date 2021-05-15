
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
use crate::model::Participant;
use crate::handle;


pub async fn start(port: u16) {
    let optional_url: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let optional_participant: Arc<Mutex<Option<Participant>>> = Arc::new(Mutex::new(None));

    let interface = warp::get()
        .and(
            warp::path::end()
            .and(warp::fs::file("./interface/build/index.html"))
            .or(warp::fs::dir("./interface/build"))
        );

    let connect = warp::path("connect")
        .and(warp::post())
        .and(with(optional_url.clone()))
        .and(warp::body::json())
        .and_then(handle::connect);

    let participate = warp::path("participate")
        .and(warp::post())
        .and(with(optional_url.clone()))
        .and(with(optional_participant.clone()))
        .and(warp::body::json())
        .and_then(handle::participate);

    let filter = interface
        .or(connect)
        .or(participate);

    warp::serve(filter)
        .run(([127, 0, 0, 1], port))
        .await;
}


fn with<T: Clone + Send>(
    reference: T
) -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || reference.clone())
}