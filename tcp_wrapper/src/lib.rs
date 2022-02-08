use crate::error::{ReceiveError, ReceiveResult, SendResult};

pub use tcp_server::server_std;
pub use tcp_server::server_async;
pub use tcp_client::client_std;
pub use tcp_client::client_async;
pub use stream::stream_std;
pub use stream::stream_async;
pub use tcp_server::server_std::connection_std;
pub use tcp_server::server_async::connection_async;
pub mod error;
mod stream;
mod tcp_client;
mod tcp_server;
