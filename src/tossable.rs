use rand::Rng;

pub trait Tossable {
    type Outcome;
    fn toss(&self) -> Self::Outcome;
    fn toss_many(&self, n: u64) -> Vec<Self::Outcome> {
        (0..n).map(|_idx| self.toss()).collect()
    }
}

pub enum CoinFace {
    Heads,
    Tails,
}

impl AsRef<str> for CoinFace {
    fn as_ref(&self) -> &str {
        match self {
            CoinFace::Heads => "heads",
            CoinFace::Tails => "tails",
        }
    }
}

pub struct Coin {
    heads_probability: u64,
}

impl Coin {
    pub fn new(heads_probability: u64) -> Self {
        Self { heads_probability }
    }
}

impl Tossable for Coin {
    type Outcome = CoinFace;

    fn toss(&self) -> Self::Outcome {
        let mut rng = rand::thread_rng();
        let outcome = rng.gen_range(0..100);
        if outcome < self.heads_probability {
            CoinFace::Heads
        } else {
            CoinFace::Tails
        }
    }
}

pub struct Dice {
    pub min: u64,
    pub max: u64,
}

impl Dice {
    pub fn new(min: u64, max: u64) -> Result<Self, &'static str> {
        if min >= max {
            return Err("Min must be > max");
        }
        Ok(Self { min, max })
    }
}

impl Tossable for Dice {
    type Outcome = u64;

    fn toss(&self) -> Self::Outcome {
        let mut rng = rand::thread_rng();
        rng.gen_range(self.min..=self.max)
    }
}
