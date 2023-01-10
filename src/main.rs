use std::net::TcpListener;

use zero2prod::{startup::run, configuration::get_configuration};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let settings = get_configuration().expect("Failed to load configuration.");
    let address = format!("127.0.0.1:{}", settings.application_port);

    let listener = TcpListener::bind(address).expect("Failed to bind random port.");
    run(listener)?.await
}