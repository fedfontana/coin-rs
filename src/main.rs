use clap::{Parser, Subcommand};
use tossable::Dice;

mod tossable;
use crate::tossable::{Coin, Tossable};

#[derive(Parser)]
struct Args {
    #[arg(default_value_t=1, value_parser=clap::value_parser!(u64).range(1..))]
    number: u64,

    #[command(subcommand)]
    tossable: TossOptions,
}

#[derive(Subcommand)]
enum TossOptions {
    Coin {
        #[arg(short='p', long, default_value_t=50, value_parser=clap::value_parser!(u64).range(0..=100))]
        heads_probability: u64,
    },
    Dice {
        #[arg(short, long, default_value_t=1, value_parser=clap::value_parser!(u64).range(1..))]
        min: u64,

        #[arg(short='M', long, default_value_t=6, value_parser=clap::value_parser!(u64).range(1..))]
        max: u64,
    },
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    match args.tossable {
        TossOptions::Coin { heads_probability } => {
            let coin = Coin::new(heads_probability);
            if args.number == 1 {
                let res = coin.toss();
                println!("Got {}", res.as_ref());
            } else {
                let res = coin.toss_many(args.number);

                let heads_count = res
                    .iter()
                    .filter(|face| matches!(face, tossable::CoinFace::Heads))
                    .count();
                let tails_count = res.len() - heads_count;

                println!(
                    "Results: {}",
                    res.iter()
                        .map(|e| e.as_ref())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                println!("Got {heads_count} heads and {tails_count} tails");
            }
        }
        TossOptions::Dice { min, max } => {
            let dice = Dice::new(min, max)?;
            if args.number == 1 {
                let res = dice.toss();
                println!("Got {res}");
            } else {
                let res = dice.toss_many(args.number);

                let mut counts = vec![0; (dice.max + 1 - dice.min) as usize];
                for r in res {
                    counts[r as usize - 1] += 1;
                }

                println!("Results:");
                for (idx, count) in counts.iter().enumerate() {
                    println!(
                        "Got {} {count} time{}",
                        idx + 1,
                        if *count != 1 { "s" } else { "" }
                    )
                }
            }
        }
    }

    Ok(())
}
