use crate::errors_trprot::{ConnectError, ConnectResult, RecvResult, SendResult};
use std::io;
use std::io::{Read, Write};
//use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::net::{SocketAddr, UdpSocket, ToSocketAddrs};
use thiserror::Error;

/// Represent STP server, that can accept incoming connections.
pub struct TrprotServer {
    udp: UdpSocket,
}

impl TrprotServer {
    /// Binds server to specefied socket.
    pub fn bind<Addrs>(addrs: Addrs) -> BindResult
    where
        Addrs: ToSocketAddrs,
    {
        let udp = UdpSocket::bind(addrs)?;
        Ok(Self { udp })
    }

    /// Blocking iterator for incoming connections.
    pub fn incoming(&self) -> impl Iterator<Item = ConnectResult<TrprotConnection>> + '_ {
        let mut buf=[0,1024];
        match self.udp.recv_from(&mut buf) {
            Ok(s) => Self::try_handshake(s),
            Err(e) => Err(ConnectError::Io(e)),
        }
    }

    fn try_handshake(mut stream: TcpStream) -> ConnectResult<TrprotConnection> {
        let mut buf = [0; 9];
        stream.read_exact(&mut buf)?;
        if &buf != b"trpclient" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        stream.write_all(b"trpserver")?;
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
    udp: UdpSocket,
}

impl TrprotConnection {
    /// Send response to client
    pub fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        crate::send_string(response, &mut self.stream)
    }

    /// Receive requests from client
    pub fn recv_request(&mut self) -> RecvResult {
        crate::recv_string(&mut self.udp)
    }

    /// Address of connected client
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.udp.peer_addr()
    }
}
