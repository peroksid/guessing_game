use std::io;
pub struct GuessAttempt {
    message: String,
}
impl GuessAttempt {
    #[must_use]
    pub fn new(message: String) -> Self {
        GuessAttempt { message }
    }

    /// Make the user to guess a number.
    /// # Panics
    /// If the function encounters I/O error which prevents from
    /// reading the line it panics.
    #[must_use]
    pub fn make(&self) -> u32 {
        println!("{}", self.message);
        let mut input = String::new();
        loop {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if let Ok(num) = input.trim().parse() {
                break num;
            }
            println!("Invalid input. Please enter a valid number.");
            input.clear();
        }
    }
}
