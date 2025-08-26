use super::attempts::Attempts;
use super::farewells::Farewell;
use super::greetings::Greeting;
use super::secrets::Secret;

pub struct GuessingGame {
    greeting: Greeting,
    secret: Secret,
    attempts: Attempts,
    farewell: Farewell,
}

impl GuessingGame {
    #[must_use]
    pub fn new(greeting: Greeting, secret: Secret, attempts: Attempts, farewell: Farewell) -> Self {
        GuessingGame {
            greeting,
            secret,
            attempts,
            farewell,
        }
    }

    pub fn play(&mut self) {
        self.greeting.execute();
        self.attempts.execute(&self.secret);
        self.farewell.execute(&self.attempts, &self.secret);
    }
}
