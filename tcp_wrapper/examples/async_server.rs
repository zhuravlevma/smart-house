use std::error::Error;
use tcp_wrapper::server_async::{Connection, TcpServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = TcpServer::bind("127.0.0.1:55331").await?;
    loop {
        let connection = server.accept().await?;
        process_connection(connection).await?
    }
}

async fn process_connection(conn: Connection) -> Result<(), Box<dyn Error>> {
    let req = conn.recv_request().await?;
    assert_eq!(req, "Hello, server");
    conn.send_response("Hello, client").await?;
    Ok(())
}
