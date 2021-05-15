mod handle;
mod model;
mod server;

#[tokio::main]
async fn main() {
    let app = clap::app_from_crate!()
        .arg(
            clap::Arg::new("port")
                .short('p')
                .long("port")
                .about("The port to serve from")
                .takes_value(true),
        );
    let matches = app.get_matches();

    let port: u16 = matches.value_of_t("port").unwrap_or(3030);
    server::start(port).await;
}
