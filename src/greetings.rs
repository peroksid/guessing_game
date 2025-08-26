pub struct Greeting {
    message: String,
}

impl Greeting {
    #[must_use]
    pub fn new(message: String) -> Self {
        Greeting { message }
    }
    pub fn execute(&self) {
        println!("{}", self.message);
    }
}
