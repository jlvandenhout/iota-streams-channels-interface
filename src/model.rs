use iota_streams::{
    app_channels::api::tangle::{
        Author,
        Subscriber,
    },
    app::transport::tangle::client::Client,
};


pub enum Participant {
    Author(Author<Client>),
    Recipient(Subscriber<Client>),
}