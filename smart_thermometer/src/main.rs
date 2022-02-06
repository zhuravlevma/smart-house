use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use udp_wrapper::UdpPusher;

fn main() -> Result<(), Box<dyn Error>> {
    let pusher = UdpPusher::new("127.0.0.1:5051".to_string());
    loop {
        let dur = Duration::from_secs(3);
        sleep(dur);
        pusher.send(27.5.to_string(), "127.0.0.1:8081".to_string());
    }
}
