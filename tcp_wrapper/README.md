# Tcp wrapper library

## Async tcp wrapper
```toml
[dependencies]
tcp_wrapper = "0.2"
tokio = { version = "1", features = ["full"] }
```
Then, on your main.rs:

```rust,no_run
use tcp_wrapper::client_async::TcpClient;
use tcp_wrapper::server_async::TcpServer;
use tcp_wrapper::server_async::Connection;
use std::thread;
use std::error::Error;
use tokio::spawn;
use tokio::join;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
     let tcp_server = TcpServer::bind("127.0.0.1:8085").await?;
     join!(
         server(tcp_server),
         client()
     );
     Ok(())
}

async fn server(server: TcpServer) -> Result<(), Box<dyn Error>> {
     let connection = server.accept().await?;
     let req = connection.recv_request().await?;
     assert_eq!(req, "Hello, server");
     connection.send_response("Hello, client").await?;
     Ok(())
}
async fn client() -> Result<(), Box<dyn Error>> {
     let mut client = TcpClient::connect("127.0.0.1:8085").await?;
     let response = client.send_request("Hello, server").await?;
     assert_eq!(response, "Hello, client");
     Ok(())
}
```

## Std tcp wrapper
```toml
[dependencies]
tcp_wrapper = "0.2"
```
Then, on your main.rs:

```rust,no_run
use tcp_wrapper::client_std::Client;
use tcp_wrapper::server_std::TcpServer;
use std::error::Error;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
   let server = TcpServer::bind("127.0.0.1:9092")?;
   thread::spawn(move || {
       for elem in server.incoming() {
           let mut e = elem.unwrap();
           let req_str = e.recv_request().unwrap();
           e.send_response("test").unwrap();
       }
    });
    let mut client = Client::connect("127.0.0.1:9092")?;
    let resp = client.send_request("hello").unwrap();
    assert_eq!(resp, "test");
    Ok(())
}
```

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/zhuravlevma/smart-house/blob/main/LICENSE
