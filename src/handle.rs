use std::sync::Arc;
use tokio::sync::Mutex;
use serde::Deserialize;
use warp::http::StatusCode;
use iota_streams::app_channels::api::tangle::{
    Author,
    Subscriber,
};
use iota_streams::app::transport::tangle::{
    PAYLOAD_BYTES,
    client::Client,
};
use crate::model::Participant;


#[derive(Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ParticipateOptions {
    Author {
        seed: String,
        multi_branching: bool
    },
    Recipient {
        seed: String,
    },
}

#[derive(Deserialize)]
pub struct ConnectOptions {
    url: String,
}

#[derive(Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum InteractOptions {
    Announce,
}


pub async fn connect(
    optional_url: Arc<Mutex<Option<String>>>,
    connect_options: ConnectOptions,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    let mut optional_url = optional_url.lock().await;
    optional_url.replace(connect_options.url);
    Ok(StatusCode::OK)
}


pub async fn participate(
    optional_url: Arc<Mutex<Option<String>>>,
    optional_participant: Arc<Mutex<Option<Participant>>>,
    participate_options: ParticipateOptions,
) -> Result<impl warp::Reply, warp::Rejection> {
    let optional_url = optional_url.lock().await;

    if let Some(url) = optional_url.as_ref() {
        let participant = match participate_options {
            ParticipateOptions::Author{seed, multi_branching} => {
                Participant::Author(
                    Author::new(
                        seed.as_str(),
                        "utf-8",
                        PAYLOAD_BYTES,
                        multi_branching,
                        Client::new_from_url(url.as_str())
                    )
                )
            },
            ParticipateOptions::Recipient{seed} => {
                Participant::Recipient(
                    Subscriber::new(
                        seed.as_str(),
                        "utf-8",
                        PAYLOAD_BYTES,
                        Client::new_from_url(url.as_str())
                    )
                )
            },
        };

        optional_participant
            .lock()
            .await
            .replace(participant);

        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::CONFLICT)
    }
}


pub async fn interact(
    optional_participant: Arc<Mutex<Option<Participant>>>,
    interact_options: InteractOptions,
) -> core::result::Result<impl warp::Reply, std::convert::Infallible> {
    let mut optional_participant = optional_participant.lock().await;

    if let Some(participant) = optional_participant.as_mut() {
        if let Participant::Author(author) = participant {
            let _ = match interact_options {
                InteractOptions::Announce => {
                    // BREAKING:
                    // author.send_announce().await
                },
            };
            Ok(StatusCode::OK)
        } else {
            Ok(StatusCode::BAD_REQUEST)
        }
    } else {
        Ok(StatusCode::CONFLICT)
    }
}
