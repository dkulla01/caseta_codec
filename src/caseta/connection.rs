use std::{str::FromStr, time::Duration};

use anyhow::{bail, Result};
use async_trait::async_trait;
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
    time::timeout,
};
use url::Host;

use super::message::Message;

#[async_trait]
pub trait TcpSocketProvider {
    async fn new_socket(&self) -> Result<TcpStream>;
}

pub struct DefaultTcpSocketProvider {
    address: Host<String>,
    port: u16,
}

impl DefaultTcpSocketProvider {
    pub fn new(address: Host<String>, port: u16) -> Self {
        Self { address, port }
    }
}

#[async_trait]
impl TcpSocketProvider for DefaultTcpSocketProvider {
    async fn new_socket(&self) -> Result<TcpStream> {
        let connection = timeout(
            Duration::from_secs(10),
            TcpStream::connect((self.address.to_string(), self.port)),
        )
        .await
        .expect("timed out connecting to caseta");
        match connection {
            Ok(socket) => anyhow::Ok(socket),
            Err(error) => bail!(error),
        }
    }
}

pub struct CasetaConnection<'a> {
    caseta_username: String,
    caseta_password: String,
    tcp_socket_provider: &'a (dyn TcpSocketProvider + 'a),
    stream: Option<BufWriter<TcpStream>>,
}

impl<'a> CasetaConnection<'a> {
    pub fn new(
        caseta_username: String,
        caseta_password: String,
        tcp_socket_provider: &'a dyn TcpSocketProvider,
    ) -> Self {
        Self {
            caseta_username,
            caseta_password,
            tcp_socket_provider,
            stream: Option::None,
        }
    }

    async fn read_frame(&mut self) -> Result<Option<Message>> {
        let stream = match self.stream {
            Some(ref mut buf_writer) => buf_writer,
            None => bail!("no writer available. is the connection still initializing?"),
        };

        let mut buffer = BytesMut::with_capacity(128);

        let read_result = stream.read_buf(&mut buffer).await;
        let num_bytes_read = read_result.expect("there was a problem reading the buffer");
        if num_bytes_read == 0 {
            if buffer.is_empty() {
                return Ok(None);
            } else {
                bail!("got an empty message from caseta");
            }
        }
        let contents = std::str::from_utf8(&buffer[..]).expect("got unparseable content");
        let message = Message::from_str(contents)
            .expect(format!("expected a valid message but got {}", contents).as_str());
        println!("got remote message {}", message);
        return Ok(Some(message));
    }

    pub async fn initialize(&mut self) -> Result<()> {
        self.stream = match self.tcp_socket_provider.new_socket().await {
            Ok(stream) => Option::Some(BufWriter::new(stream)),
            Err(error) => bail!(error),
        };

        self.log_in().await
    }

    async fn log_in(&mut self) -> Result<()> {
        let contents = self.read_frame().await;
        match contents {
            Ok(Some(Message::LoginPrompt)) => println!("received the login prompt"),
            Ok(Some(unexpected_message)) => {
                bail!("got a weird random message: {:?}", unexpected_message);
            }
            Ok(None) => {
                bail!("got an empty message");
            }
            Err(e) => {
                bail!("got an error: {:?}", e);
            }
        }
        self.write(format!("{}\r\n", self.caseta_username).as_str())
            .await?;
        let contents = self.read_frame().await;
        match contents {
            Ok(Some(Message::PasswordPrompt)) => println!("got password prompt"),
            Ok(Some(unexpected_message)) => {
                bail!("got a weird random message: {:?}", unexpected_message);
            }
            Ok(None) => {
                bail!("got an empty message");
            }
            Err(e) => {
                bail!("got an error: {:?}", e);
            }
        }
        if let Ok(()) = self
            .write(format!("{}\r\n", self.caseta_password).as_str())
            .await
        {
        } else {
            bail!("got an error logging in");
        }

        let contents = self.read_frame().await;

        match contents {
            Ok(Some(Message::LoggedIn)) => {
                return Ok(());
            }
            _ => {
                bail!("Did not receive the expected GNET> message")
            }
        }
    }

    async fn write(&mut self, message: &str) -> Result<()> {
        let stream = match self.stream {
            Some(ref mut buf_writer) => buf_writer,
            None => bail!("has the connection been initialized?"),
        };
        let outcome = stream.write(message.as_bytes()).await;

        match outcome {
            Ok(_) => {}
            Err(e) => {
                bail!("couldn't flush the socket read/write buffer: {:?}", e)
            }
        }

        let outcome = stream.flush().await;
        match outcome {
            Ok(_) => Ok(()),
            Err(e) => {
                bail!("couldn't flush the socket read/write buffer: {:?}", e)
            }
        }
    }
    pub async fn await_message(&mut self) -> Result<Message> {
        let message = self.read_frame().await;
        match message {
            Ok(Some(content)) => Ok(content),
            Ok(None) => bail!("it looks like we've disconnected from caseta"),
            Err(err) => {
                bail!(
                    "we ran into a problem waiting for the next message: {}",
                    err
                )
            }
        }
    }
}
