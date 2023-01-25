use std::env;

use anyhow::{bail, Result};
use caseta_codec::caseta::connection::{CasetaConnection, DefaultTcpSocketProvider};
use url::Host;

#[tokio::main]
async fn main() -> Result<()> {
    let caseta_host_string = env::var("CASETA_HOST").expect("expected a CASETA_HOST env var");
    let caseta_port_string = env::var("CASETA_PORT").expect("expected a CASETA_PORT env var");
    let caseta_username = env::var("CASETA_USERNAME").expect("expected a CASETA_USERNAME env var");
    let caseta_password = env::var("CASETA_PASSWORD").expect("expected a CASETA_PASSWORD env var");

    let caseta_host = Host::parse(&caseta_host_string)
        .expect(format!("{} is not a valid host", caseta_host_string).as_str());

    let caseta_port: u16 = caseta_port_string
        .parse::<u16>()
        .expect(format!("{} is not a valid port", caseta_port_string).as_str());

    let tcp_socket_provider = DefaultTcpSocketProvider::new(caseta_host, caseta_port);
    let mut caseta_connection =
        CasetaConnection::new(caseta_username, caseta_password, &tcp_socket_provider);
    caseta_connection.initialize().await?;
    loop {
        let message = caseta_connection.await_message().await;
        match message {
            Ok(contents) => print!("got the following message: `{}`", contents),
            Err(e) => bail!("ran into an error waiting for the next message: {}", e),
        }
    }
}
