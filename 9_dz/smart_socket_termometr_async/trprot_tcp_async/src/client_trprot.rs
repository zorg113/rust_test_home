use crate::errors_trprot::{ConnectError, ConnectResult, RecvError, SendError};
use tokio ::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs}
};

use thiserror::Error;

pub struct TrprotClient {
    stream: TcpStream,
}

impl TrprotClient {
    pub async fn connect<Addrs>(addrs: Addrs) -> ConnectResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs).await?;
        Self::try_handsnake(stream)
    }

    pub async fn send_request<R: AsRef<str>>(&mut self, req: R) -> RequestResult {
        crate::send_string(req, &mut self.stream).await?;
        let response = crate::recv_string(&mut self.stream).await?;
        Ok(response)
    }

    fn async try_handsnake(mut stream: TcpStream) -> ConnectResult<Self> {
        stream.write_all(b"trpclient").await?;
        let mut buf = [0; 9];
        stream.read_exact(&mut buf).await?;
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
