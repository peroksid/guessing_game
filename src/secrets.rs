use rand::Rng;

pub struct Secret {
    pub number: u32,
}

impl Secret {
    #[must_use]
    pub fn new(range: std::ops::RangeInclusive<u32>) -> Self {
        let number = rand::rng().random_range(range);
        Secret { number }
    }
    #[must_use]
    pub fn is_correct(&self, guess: u32) -> bool {
        self.number == guess
    }
}
