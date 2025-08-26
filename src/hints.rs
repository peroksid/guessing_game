use super::secrets::Secret;
pub struct Hint {
    message: String,
}
impl Hint {
    #[must_use]
    pub fn new(message: String) -> Self {
        Hint { message }
    }

    pub fn provide(&self, secret: &Secret, guess: u32) {
        match guess.cmp(&secret.number) {
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Less => {
                println!("{}: The number is higher than {}.", self.message, guess);
            }
            std::cmp::Ordering::Greater => {
                println!("{}: The number is lower than {}.", self.message, guess);
            }
        }
    }
}
