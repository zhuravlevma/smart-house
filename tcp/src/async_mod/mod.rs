mod client;
mod server;

use std::error::Error;
use std::io;
use tokio::net::{TcpStream, ToSocketAddrs};
use crate::error::{ConnectResult, RequestError, RequestResult};
use crate::{ReceiveError, ReceiveResult, SendResult};

struct Stream {
    pub tcp: TcpStream,
}

impl Stream {
    pub async fn new(tcp_stream: TcpStream) -> Self {
        Self {
            tcp: tcp_stream,
        }
    }

    pub async fn write_all_async(&self, stream: &TcpStream, buf: &[u8]) -> io::Result<()> {
        let mut written = 0;

        while written < buf.len() {
            stream.writable().await?;

            match stream.try_write(&buf[written..]) {
                Ok(0) => break,
                Ok(n) => {
                    written += n;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    async fn send_string_async<D: AsRef<str>>(&self, stream: &TcpStream, d: D) -> SendResult {
        let bytes = d.as_ref().as_bytes();
        let len = bytes.len() as u32;
        let len_bytes = len.to_be_bytes();
        self.write_all_async(stream,&len_bytes).await?;
        self.write_all_async(stream,bytes).await?;
        Ok(())
    }

    async fn recv_string_async(&self) -> ReceiveResult {
        let mut buf = [0; 4];
        self.read_exact_async(&mut buf).await?;
        let len = u32::from_be_bytes(buf);

        let mut buf = vec![0; len as _];
        self.read_exact_async(&mut buf).await?;
        String::from_utf8(buf).map_err(|_| ReceiveError::BadEncoding)
    }
}