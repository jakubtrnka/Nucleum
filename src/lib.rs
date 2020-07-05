pub mod error;

use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::prelude::*;
use tracing::*;

use error::Result;
use std::fmt::Debug;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct NucleumServer<T: ToSocketAddrs> {
    listen_on: T,
}

impl<T> NucleumServer<T>
where
    T: ToSocketAddrs + Debug,
{
    pub fn new(listen_on: T) -> Self {
        Self { listen_on }
    }

    #[tracing::instrument]
    pub async fn run(&self) -> Result<()> {
        let mut listener = TcpListener::bind(&self.listen_on).await?;
        event!(
            Level::INFO,
            server = listener.local_addr().map(|a| a.to_string())?.as_str()
        );
        while let Ok((tcp_stream, socket_addr)) = listener.accept().await {
            event!(Level::INFO, peer = socket_addr.to_string().as_str());
            let connection = Connection::new(tcp_stream, socket_addr);
            tokio::spawn(async move {
                connection.handle().await;
            });
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Connection {
    tcp_stream: TcpStream,
    downstream_socket_addr: SocketAddr,
}

impl Connection {
    fn new(tcp_stream: TcpStream, downstream_socket_addr: SocketAddr) -> Self {
        Self {
            tcp_stream,
            downstream_socket_addr,
        }
    }

    #[tracing::instrument]
    async fn handle(mut self) {
        let mut str_buff = String::new();
        while let Ok(bytes_read) = self.tcp_stream.read_to_string(&mut str_buff).await {
            if bytes_read == 0 {
                event!(Level::INFO, "TCP stream finished.");
                break;
            }
        }
    }
}
