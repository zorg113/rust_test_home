use crate::errors_trprot::{ConnectError, ConnectResult, RecvError, SendError};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use thiserror::Error;

pub struct TrprotClient {
    stream: TcpStream,
}

impl TrprotClient {
    pub fn connect<Addrs>(addrs: Addrs) -> ConnectResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs)?;
        Self::try_handsnake(stream)
    }

    pub fn send_request<R: AsRef<str>>(&mut self, req: R) -> RequestResult {
        crate::send_string(req, &mut self.stream)?;
        let response = crate::recv_string(&mut self.stream)?;
        Ok(response)
    }

    fn try_handsnake(mut stream: TcpStream) -> ConnectResult<Self> {
        stream.write_all(b"trpclient")?;
        let mut buf = [0; 9];
        stream.read_exact(&mut buf)?;
        if &buf != b"trpserver" {
            let msg = format!("recived: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        Ok(Self { stream })
    }
}

pub type RequestResult = Result<String, RequestError>;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error(transparent)]
    Recv(#[from] RecvError),
}
