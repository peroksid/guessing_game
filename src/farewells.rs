use super::attempts::Attempts;
use super::secrets::Secret;
pub struct Farewell {
    message: String,
}
impl Farewell {
    #[must_use]
    pub fn new(message: String) -> Self {
        Farewell { message }
    }
    pub fn execute(&self, attempts: &Attempts, secret: &Secret) {
        if attempts.final_success {
            println!(
                "{} You guessed the number in {} attempts.",
                self.message, attempts.count
            );
        } else {
            println!("The number was {}.", secret.number);
        }
        println!("{}", self.message);
    }
}
