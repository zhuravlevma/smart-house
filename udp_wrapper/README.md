# Udp wrapper library

## Async udp server
```toml
[dependencies]
udp_wrapper = "0.2"
tokio = { version = "1", features = ["full"] }
```
Then, on your main.rs:

```rust,no_run
async fn main() -> Result<(), Box<dyn Error>> {
    let server = UdpServerAsync::new("127.0.0.1:9001".to_string()).await?;
    let (_usize, address, data) = server.receive().await?;
    server.response(data, address);
}
```

## Udp std server
```toml
[dependencies]
udp_wrapper = "0.2"
tokio = { version = "1", features = ["full"] }
```
Then, on your main.rs:

```rust,no_run
use udp_wrapper::UdpServer;
use std::error::Error;
async fn main() -> Result<(), Box<dyn Error>> {
    let server = UdpServer::new("127.0.0.1:9001".to_string())?;
    server.receive()
}
```

## Udp client (client - server)
```toml
[dependencies]
udp_wrapper = "0.2"
tokio = { version = "1", features = ["full"] }
```
Then, on your main.rs:

```rust,no_run
async fn main() -> Result<(), Box<dyn Error>> {
    let client = UdpClient::new("127.0.0.1:9001".to_string())?;
    let response = client.send("Hello, I'm a client".to_string(), "127.0.0.1:9001".to_string())?;
}
```

## Udp pusher (without response)
```toml
[dependencies]
udp_wrapper = "0.2"
tokio = { version = "1", features = ["full"] }
```
Then, on your main.rs:

```rust,no_run
use udp_wrapper::UdpPusher;
use std::error::Error;
async fn main() -> Result<(), Box<dyn Error>> {
    let server = UdpPusher::new("127.0.0.1:9001".to_string())?;
    server.send("Hello, I'm pusher".to_string(), "127.0.0.1:8081".to_string())?;
    Ok(())
}
```

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/zhuravlevma/smart-house/blob/main/LICENSE
