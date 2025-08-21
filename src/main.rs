use guessing_game::{Attempts, Farewell, Greeting, GuessAttempt, GuessingGame, Hint, Secret};

fn main() {
    let mut guessing_game = GuessingGame::new(
        Greeting::new(String::from("Welcome to the Guessing Game!")),
        Secret::new(1..=100),
        Attempts::new(
            5,
            GuessAttempt::new(String::from("Please enter your guess: ")),
            Hint::new(String::from("Here's a hint: ")),
        ),
        Farewell::new(String::from("Thanks for playing!")),
    );
    guessing_game.play();
}
