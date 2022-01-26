use std::fs;

pub struct Config {
    pub url: String,
}

impl Config {
    pub fn new(host: String, port: String) -> Result<Config, &'static str> {
        Ok(Config {
            url: fs::read_to_string("settings/addr")
                .unwrap_or_else(|_| format!("{}:{}", host, port)),
        })
    }
}
