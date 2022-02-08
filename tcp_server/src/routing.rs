use crate::tcp_request::Request;

pub trait Routing {
    fn routing(&mut self, request: Request) -> String;
}
