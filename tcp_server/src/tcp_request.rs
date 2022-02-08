use std::error::Error;
use std::str::Split;

pub struct ConnectionRequest<'a>(Split<'a, &'a str>);
impl<'a> ConnectionRequest<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s.split("|||"))
    }

    pub fn next_data(&mut self) -> &'a str {
        self.0.next().unwrap_or("")
    }
}

pub struct Request {
    resource: String,
    data: String,
}

impl Request {
    pub fn new(req_str: &str) -> Result<Self, Box<dyn Error>> {
        let mut request = ConnectionRequest::new(req_str);
        let resource = String::from(request.next_data());
        let data = String::from(request.next_data());
        Ok(Self { resource, data })
    }
}

impl Request {
    pub fn get_data(&self) -> &str {
        &self.data
    }

    pub fn get_resource(&self) -> &str {
        &self.resource
    }
}
