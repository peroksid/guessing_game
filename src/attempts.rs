use super::guess_attempts::GuessAttempt;
use super::hints::Hint;
use super::secrets::Secret;
pub struct Attempts {
    count_limit: u32,
    pub count: u32,
    guess_attempt: GuessAttempt,
    pub final_success: bool,
    hint: Hint,
}
impl Attempts {
    #[must_use]
    pub fn new(count_limit: u32, guess_attempt: GuessAttempt, hint: Hint) -> Self {
        Attempts {
            count_limit,
            count: 0,
            guess_attempt,
            final_success: false,
            hint,
        }
    }

    pub fn execute(&mut self, secret: &Secret) {
        for _ in 0..self.count_limit {
            self.count += 1;
            let guess = self.guess_attempt.make();
            if secret.is_correct(guess) {
                self.final_success = true;
                break;
            }
            self.hint.provide(secret, guess);
            println!(
                "Incorrect guess. You have {} attempts left.",
                self.count_limit - self.count
            );
        }
    }
}
