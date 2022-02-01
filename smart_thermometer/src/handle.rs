pub struct Handle {}

impl Handle {
    pub fn new() -> Self { Self {} }
    pub fn routing(&self, msg: String) -> String {
        println!("{}", msg);
        35.to_string()
    }
}
