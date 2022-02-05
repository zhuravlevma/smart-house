use tokio::net::{TcpStream, ToSocketAddrs};
use crate::async_mod::Stream;
use crate::error::{ConnectError, ConnectResult};

struct Client {
    stream: Stream,
}

impl Client {
    pub async fn connect<Addrs>(addrs: Addrs) -> ConnectResult<Client> where Addrs: ToSocketAddrs {
        let stream = TcpStream::connect(addrs).await?;
        let stream = Stream::new(stream).await;
        let client = Self {stream};
        client.try_handshake().await?;
        Ok(client)
    }

    async fn try_handshake(&self) -> ConnectResult<()> {
        self.stream.write_all_async(b"clnt").await?;
        let mut buf = [0; 4];
        self.stream.read_exact_async(&mut buf).await?;
        if &buf != b"serv" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        Ok(())
    }
}
