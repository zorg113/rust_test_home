use crate::error::{ConnectError, ConnectResult, RecvError, SendError};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use thiserror::Error;

pub struct TrprotClient{
    stream: TcpStream,
}

impl TrprotClient {
    
}