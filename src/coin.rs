use rand::Rng;

pub trait Tossable {
    type Outcome;
    fn toss(&self) -> Self::Outcome;
    fn toss_many(&self, n: u64) -> Vec<Self::Outcome> {
        (0..n).map(|_idx| self.toss()).collect()
    }
}


pub enum CoinFace {
    Head,
    Tails,
}

impl AsRef<str> for CoinFace {
    fn as_ref(&self) -> &str {
        match self {
            CoinFace::Head => "head",
            CoinFace::Tails => "tails",
        }
    }
}

pub struct Coin {}

impl Tossable for Coin {
    type Outcome = CoinFace;

    fn toss(&self) -> Self::Outcome {
        let mut rng = rand::thread_rng();
        let outcome: bool = rng.gen();
        match outcome {
            true => CoinFace::Head,
            false => CoinFace::Tails,
        }
    }
}