mod cli;
mod http_server;
mod templates;

use cli::clap_parser;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let args = clap_parser();

    let port = args.get_one::<u16>("port").expect("port required");
    let bind = args.get_one::<String>("bind").expect("required");
    let dir = args.get_one::<String>("dir").expect("required");

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    http_server::serve(&bind, *port, dir.to_owned()).await;
}
