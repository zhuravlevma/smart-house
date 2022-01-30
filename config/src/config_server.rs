use std::fs;

pub struct ConfigServer {
    pub url: String,
}

impl ConfigServer {
    pub fn new(host: String, port: String) -> Result<ConfigServer, String> {
        Ok(Self {
            url: fs::read_to_string("settings/addr")
                .unwrap_or_else(|_| format!("{}:{}", host, port)),
        })
    }
}
