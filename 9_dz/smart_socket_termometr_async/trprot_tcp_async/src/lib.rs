use crate::errors_trprot::{RecvError, RecvResult, SendResult};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub mod client_trprot;
pub mod errors_trprot;
pub mod server_trprot;

async fn send_string<Data: AsRef<str>, Writer: AsyncWriteExt + Unpin>(
    data: Data,
    mut writer: Writer,
) -> SendResult {
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes).await?;
    writer.write_all(bytes).await?;
    Ok(())
}

async fn recv_string<Reader: AsyncReadExt + Unpin>(mut reader: Reader) -> RecvResult {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf).await?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf).await?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}
