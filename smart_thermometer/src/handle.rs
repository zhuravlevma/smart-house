pub struct Handle {}

impl Handle {
    pub fn new() -> Self {
        Self {}
    }
    pub fn routing(&self, msg: String) -> String {
        println!("Start: {}", msg);
        for _i in 0..10000000 {
            _i.to_string();
        }
        println!("end");
        35.to_string()
    }
}
