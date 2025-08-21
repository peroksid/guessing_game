use std::io;
use rand::Rng;


struct Greeting {
    message: String,
}
impl Greeting {
    fn new(message: String) -> Self {
        Greeting { message }
    }
    fn execute(&self) {
        println!("{}", self.message);
    }
}
struct Secret {
    number: u32,
}

impl Secret {
    fn new(range: std::ops::RangeInclusive<u32>) -> Self {
        let number = rand::rng().random_range(range);
        Secret { number }
    }
    fn is_correct(&self, guess: u32) -> bool {
        self.number == guess
    }
}

struct GuessAttempt {
    message: String,
}
impl  GuessAttempt {
    fn new(message: String) -> Self {
        GuessAttempt { message }
    }

    fn make(&self) -> u32 {
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

struct Hint {
    message: String,
} 
impl Hint {
    fn new(message: String) -> Self {
        Hint { message }
    }

    fn provide(&self, secret: &Secret, guess: u32) {
        if guess < secret.number {
            println!("{}: The number is higher than {}.", self.message, guess);
        } else if guess > secret.number {
            println!("{}: The number is lower than {}.", self.message, guess);
        }
    }
}
struct Attempts {
    count_limit: u32,
    count: u32,
    guess_attempt: GuessAttempt,
    final_success: bool,
    hint:Hint,
}
impl Attempts {
    fn new(count_limit: u32, guess_attempt: GuessAttempt, hint: Hint) -> Self {
        Attempts {
            count_limit,
            count: 0,
            guess_attempt,
            final_success: false,
            hint,
        }
    }

    fn execute(&mut self, secret: &Secret) {
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

struct Farewell{
    message: String,
}
impl Farewell {
    fn new(message: String) -> Self {
        Farewell { message }
    }
    fn execute(&self, attempts: &Attempts, secret: &Secret) {
        if attempts.final_success {
            println!("{} You guessed the number in {} attempts.", self.message, attempts.count);
        } else {
            println!("The number was {}.", secret.number);
        }
        println!("{}", self.message);
    }
}
struct GuessingGame {
    greeting: Greeting,
    secret: Secret,
    attempts: Attempts,
    farewell: Farewell,
}

impl GuessingGame {
    fn new(greeting: Greeting, secret: Secret, attempts: Attempts, farewell: Farewell) -> Self {
        GuessingGame {
            greeting,
            secret,
            attempts,
            farewell,
        }
    }

    fn play(&mut self) {
        self.greeting.execute();
        self.attempts.execute(&self.secret);
        self.farewell.execute(&self.attempts, &self.secret);
    }
}


fn main() {
    let mut guessing_game = GuessingGame::new(
        Greeting::new(String::from("Welcome to the Guessing Game!")),
        Secret::new(1..=100),
        Attempts::new(
            5,
            GuessAttempt::new(String::from("Please enter your guess: ")),
            Hint::new(String::from("Here's a hint: "))
        ),
        Farewell::new(String::from("Thanks for playing!")),
    );
    guessing_game.play();
}
