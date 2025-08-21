


mod secrets {
    use rand::Rng;
    pub struct Secret {
        pub number: u32,
    }

    impl Secret {
        pub fn new(range: std::ops::RangeInclusive<u32>) -> Self {
            let number = rand::rng().random_range(range);
            Secret { number }
        }
        pub fn is_correct(&self, guess: u32) -> bool {
            self.number == guess
        }
    }
}

mod greetings {
    pub struct Greeting {
        message: String,
    }
    impl Greeting {
        pub fn new(message: String) -> Self {
            Greeting { message }
        }
        pub fn execute(&self) {
            println!("{}", self.message);
        }
    }
}

mod guess_attempts {
    use std::io;
    pub struct GuessAttempt {
        message: String,
    }
    impl  GuessAttempt {
        pub fn new(message: String) -> Self {
            GuessAttempt { message }
        }

        pub fn make(&self) -> u32 {
            println!("{}", self.message);
            let mut input = String::new();
            loop {
                io::stdin().read_line(&mut input).expect("Failed to read line");
                match input.trim().parse() {
                    Ok(num) => break num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid number.");
                        input.clear();
                    }
                }
            }
        }
    }
}

mod hints {
    use super::secrets::Secret;
    pub struct Hint {
        message: String,
    } 
    impl Hint {
        pub fn new(message: String) -> Self {
            Hint { message }
        }

        pub fn provide(&self, secret: &Secret, guess: u32) {
            if guess < secret.number {
                println!("{}: The number is higher than {}.", self.message, guess);
            } else if guess > secret.number {
                println!("{}: The number is lower than {}.", self.message, guess);
            }
        }
    }
}

mod attempts {
    use super::guess_attempts::GuessAttempt;
    use super::hints::Hint;
    use super::secrets::Secret;
    pub struct Attempts {
        count_limit: u32,
        pub count: u32,
        guess_attempt: GuessAttempt,
        pub final_success: bool,
        hint:Hint,
    }
    impl Attempts {
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
                } else {
                    self.hint.provide(secret, guess);
                    println!("Incorrect guess. You have {} attempts left.", self.count_limit - self.count);
                }
            }
        }
    }
}

mod farewells {
    use super::attempts::Attempts;
    use super::secrets::Secret;
    pub struct Farewell{
        message: String,
    }
    impl Farewell {
        pub fn new(message: String) -> Self {
            Farewell { message }
        }
        pub fn execute(&self, attempts: &Attempts, secret: &Secret) {
            if attempts.final_success {
                println!("{} You guessed the number in {} attempts.", self.message, attempts.count);
            } else {
                println!("The number was {}.", secret.number);
            }
            println!("{}", self.message);
        }
    }
}

mod guessing_game {
    use super::greetings::Greeting;
    use super::secrets::Secret;
    use super::attempts::Attempts;
    use super::farewells::Farewell;

    pub struct GuessingGame {
        greeting: Greeting,
        secret: Secret,
        attempts: Attempts,
        farewell: Farewell,
    }

    impl GuessingGame {
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
}

pub use guessing_game::GuessingGame;
pub use greetings::Greeting;
pub use secrets::Secret;
pub use guess_attempts::GuessAttempt;
pub use hints::Hint;
pub use attempts::Attempts;
pub use farewells::Farewell;
