mod client;
mod server;

use std::error::Error;
use std::io;
use tokio::net::{TcpStream, ToSocketAddrs};
use crate::error::{ConnectResult, RequestError, RequestResult};
use crate::{ReceiveError, ReceiveResult, SendResult};

struct Stream {
    tcp: TcpStream,
}

impl Stream {
    pub async fn new(tcp_stream: TcpStream) -> Self {
        Self {
            tcp: tcp_stream,
        }
    }
    pub async fn read_exact_async(&self, buf: &mut [u8]) -> io::Result<()> {
        let mut red = 0;
        while red < buf.len() {
            self.tcp.readable().await?;
            match self.tcp.try_read(&mut buf[red..]) {
                Ok(0) => break,
                Ok(n) => {
                    red += n;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    pub async fn write_all_async(&self, buf: &[u8]) -> io::Result<()> {
        let mut written = 0;

        while written < buf.len() {
            self.tcp.writable().await?;

            match self.tcp.try_write(&buf[written..]) {
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

    async fn send_string_async<D: AsRef<str>>(&self, d: D) -> SendResult {
        let bytes = d.as_ref().as_bytes();
        let len = bytes.len() as u32;
        let len_bytes = len.to_be_bytes();
        self.write_all_async(&len_bytes).await?;
        self.write_all_async(bytes).await?;
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