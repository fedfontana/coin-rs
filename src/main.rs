use clap::Parser;

use crate::coin::{ Coin, Tossable };

mod coin;

#[derive(Parser)]
struct Args {
    #[arg(short, default_value_t=1, value_parser=clap::value_parser!(u64).range(1..))]
    number: u64
}

fn main() {
    let args = Args::parse();
    let coin = Coin {};
    let res = coin.toss_many(args.number);
    let heads_count = res.iter().filter(|face| matches!(face, coin::CoinFace::Head)).count();
    let tails_count = res.len() - heads_count;
    println!("Results: {}", res.iter().map(|e| e.as_ref()).collect::<Vec<_>>().join(", "));
    println!("Got {heads_count} heads and {tails_count} tails");
}
