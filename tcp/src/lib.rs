use crate::error::{ReceiveError, ReceiveResult, SendResult};
use std::io::{Read, Write};
use std::sync::mpsc::RecvError;

mod client;
mod error;
mod server;

struct Stream {}

impl Stream {
    fn send_string<D: AsRef<str>, W: Write>(d: D, mut w: W) -> SendResult {
        let bytes = d.as_ref().as_bytes();
        let len = bytes.len() as u32;
        let len_bytes = len.to_be_bytes();
        w.write_all(&len_bytes)?;
        w.write_all(bytes)?;
        Ok(())
    }
    fn receive_string<R: Read>(mut r: R) -> ReceiveResult {
        let mut buf = [0; 4];
        r.read_exact(&mut buf)?;
        let len = u32::from_be_bytes(buf);
        let mut buf = vec![0; len as _];
        r.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(|_| ReceiveError::BadEncoding)
    }
}
