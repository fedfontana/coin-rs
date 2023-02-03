use std::collections::HashMap;

use clap::{Parser, Subcommand};
use tossable::{Chooser, Dice, Extractor};

mod tossable;
use crate::tossable::{Coin, Tossable};

#[derive(Parser)]
struct Args {
    /// number of times you want to toss that item
    #[arg(default_value_t=1, value_parser=clap::value_parser!(u64).range(1..))]
    number: u64,

    /// If true, the program will print all of the results in order.
    /// Works only if NUMBER > 1
    #[arg(long, global(true))]
    print: bool,

    /// the item you want to toss
    #[command(subcommand)]
    tossable: Option<TossOptions>,
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

    /// Extract with replacement
    Choose {
        #[arg(num_args=2..)]
        choices: Vec<String>,
    },

    /// Extract without replacement
    Extract {
        #[arg(num_args=2..)]
        choices: Vec<String>,
    },
}

impl Default for TossOptions {
    fn default() -> Self {
        Self::Coin {
            heads_probability: 50,
        }
    }
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    match args.tossable.unwrap_or_default() {
        TossOptions::Coin { heads_probability } => {
            let mut coin = Coin::new(heads_probability);
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

                if args.print {
                    println!(
                        "Results: {}",
                        res.iter()
                            .map(|e| e.as_ref())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }

                println!("Got {heads_count} heads and {tails_count} tails");
            }
        }
        TossOptions::Dice { min, max } => {
            let mut dice = Dice::new(min, max)?;
            if args.number == 1 {
                let res = dice.toss();
                println!("Got {res}");
            } else {
                let res = dice.toss_many(args.number);

                let mut counts = vec![0; (dice.max + 1 - dice.min) as usize];
                for r in res.iter() {
                    counts[*r as usize - 1] += 1;
                }

                if args.print {
                    println!(
                        "Results: {}",
                        res.iter()
                            .map(|r| r.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }

                for (idx, count) in counts.iter().enumerate() {
                    println!(
                        "Got {} {count} time{}",
                        idx + 1,
                        if *count != 1 { "s" } else { "" }
                    )
                }
            }
        }
        TossOptions::Choose { choices } => {
            let mut chooser = Chooser::new(choices);
            if args.number == 1 {
                let res = chooser.toss();
                println!("Got {res}");
            } else {
                let res = chooser.toss_many(args.number);

                let mut counts = HashMap::<&str, u64>::new();
                for r in res.iter() {
                    counts
                        .entry(r)
                        .and_modify(|e| {
                            *e += 1;
                        })
                        .or_insert(1);
                }

                if args.print {
                    println!("Results: {}", res.join(", "));
                }

                for (item, count) in counts.iter() {
                    println!(
                        "Got {item} {count} time{}",
                        if *count != 1 { "s" } else { "" }
                    )
                }
            }
        }
        TossOptions::Extract { choices } => {
            if args.number as usize >= choices.len() {
                return Err("`count` must be smaller than the amount of choices.");
            }

            let mut extractor = Extractor::new(choices);
            if args.number == 1 {
                let res = extractor.toss();
                println!("Got {res}");
            } else {
                let res = extractor.toss_many(args.number);
                println!("Got {}", res.join(", "));
            }
        }
    }

    Ok(())
}
