use std::{io, net::SocketAddr};

use crate::errors_trprot::{ConnectError, ConnectResult, RecvResult, SendResult};
use thiserror::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

/// Represent STP server, that can accept incoming connections.
pub struct TrprotServer {
    tcp: TcpListener,
}

impl TrprotServer {
    /// Binds server to specefied socket.
    pub async fn bind<Addrs>(addrs: Addrs) -> BindResult
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs).await.unwrap();
        Ok(Self { tcp })
    }

    /// Blocking iterator for incoming connections.
    pub async fn incoming(&self) -> ConnectResult<TrprotConnection> {
        match self.tcp.accept().await {
            Ok((_soket, _)) => Self::try_handshake(_soket).await,
            Err(e) => Err(ConnectError::Io(e)),
        }
    }

    async fn try_handshake(mut stream: TcpStream) -> ConnectResult<TrprotConnection> {
        let mut buf = [0; 9];
        stream.read_exact(&mut buf).await?;
        if &buf != b"trpclient" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        stream.write_all(b"trpserver").await?;
        Ok(TrprotConnection { stream })
    }
}

pub type BindResult = Result<TrprotServer, BindError>;

/// Bind to socket error
#[derive(Debug, Error)]
pub enum BindError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

/// Represent connection from client.
///
/// Allows to receive requests and send responses.
pub struct TrprotConnection {
    stream: TcpStream,
}

impl TrprotConnection {
    /// Send response to client
    pub async fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        crate::send_string(response, &mut self.stream).await
    }

    /// Receive requests from client
    pub async fn recv_request(&mut self) -> RecvResult {
        crate::recv_string(&mut self.stream).await
    }

    /// Address of connected client
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
