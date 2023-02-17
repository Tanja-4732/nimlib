//! A CLI for the `nimlib` crate
//!
//! See
//!
//! ```bash
//! nimlib --help
//! nimlib nimber --help
//! nimlib splits --help
//! nimlib make-rule-set --help
//! ```

#![deny(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use clap::{Args, Parser};
use nimlib::{nimbers, NimRule, Split, Stack, TakeSize};

#[derive(clap::Parser)]
#[command(about = "A Nim-game CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {
    #[command(about = "Calculate the nimber for a pile of given height")]
    Nimber,
    #[command(about = "Calculate all possible splits for a given height")]
    Splits {
        #[arg(help = "Height of the stack to calculate splits for")]
        height: u64,
        #[arg(short, long, help = "Output as CSV")]
        csv: bool,
    },
    #[command(about = "Create a JSON rule set using CLI parameters")]
    MakeRuleSet(MakeRuleSet),
}

#[derive(Args)]
struct MakeRuleSet {
    /// A list of heights which remainder cannot be split
    #[arg(long, short = 'n')]
    take_split_never: Vec<u64>,

    /// A list of heights which remainder can be split optionally
    #[arg(long, short = 'o')]
    take_split_optional: Vec<u64>,

    /// A list of heights which remainder must be split
    #[arg(long, short = 'a')]
    take_split_always: Vec<u64>,

    /// Allow for taking arbitrary amounts of coins (split(s): never, optional, always)
    #[arg(long, short = 's')]
    allow_any_take: Option<Split>,

    /// Allow for placing arbitrary amounts of coins (to be implemented)
    #[arg(long, short = 'p')]
    allow_place: bool,

    /// Pretty-print the JSON output
    #[arg(long, short = 'P')]
    pretty_print: bool,
}

/// The main function of the nimlib CLI
///
/// Parses the CLI arguments and handles the subcommands
pub fn main() {
    let args = Cli::parse();
    match args.action {
        Action::Nimber => {
            println!()
        }
        Action::Splits { height, csv } => {
            let splits = nimbers::calculate_splits(height);

            if csv {
                println!("left,right");
                for (Stack(left), Stack(right)) in splits {
                    println!("{left},{right}");
                }
                return;
            }

            if splits.is_empty() {
                println!("No splits for height {height}");
                return;
            }

            println!("Splits for height {height}:");

            let max_digits_left = splits[splits.len() - 1].0 .0.ilog10() as usize + 1;
            let max_digits_right = splits[0].1 .0.ilog10() as usize + 1;

            for (Stack(left), Stack(right)) in splits {
                println!("{left:max_digits_left$} + {right:max_digits_right$}");
            }
        }
        Action::MakeRuleSet(MakeRuleSet {
            take_split_never,
            take_split_optional,
            take_split_always,
            allow_any_take,
            allow_place,
            pretty_print,
        }) => {
            let mut rule_set: Vec<NimRule> = Default::default();

            if !take_split_never.is_empty() {
                rule_set.push(NimRule {
                    take: TakeSize::List(take_split_never),
                    split: Split::Never,
                });
            }

            if !take_split_optional.is_empty() {
                rule_set.push(NimRule {
                    take: TakeSize::List(take_split_optional),
                    split: Split::Optional,
                });
            }

            if !take_split_always.is_empty() {
                rule_set.push(NimRule {
                    take: TakeSize::List(take_split_always),
                    split: Split::Always,
                });
            }

            match allow_any_take {
                Some(Split::Never) => {
                    rule_set.push(NimRule {
                        take: TakeSize::Any,
                        split: Split::Never,
                    });
                }
                Some(Split::Optional) => {
                    rule_set.push(NimRule {
                        take: TakeSize::Any,
                        split: Split::Optional,
                    });
                }
                Some(Split::Always) => {
                    rule_set.push(NimRule {
                        take: TakeSize::Any,
                        split: Split::Always,
                    });
                }
                None => {}
            }

            if allow_place {
                rule_set.push(NimRule {
                    take: TakeSize::Place,
                    split: Split::Never,
                });
            }

            println!(
                "Made rule set:\n{}",
                if pretty_print {
                    serde_json::to_string_pretty(&rule_set).unwrap()
                } else {
                    serde_json::to_string(&rule_set).unwrap()
                }
            );
        }
    }
}
