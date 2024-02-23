use std::{io::Error, net::TcpListener};

use rinha::{startup};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let address = "127.0.0.1:3000";
    let listener = TcpListener::bind(address)?;

    startup::run(listener)?.await
}
