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
    #[arg(long, short = 'n')]
    take_split_never: Vec<u64>,

    #[arg(long, short = 'o')]
    take_split_optional: Vec<u64>,

    #[arg(long, short = 'a')]
    take_split_always: Vec<u64>,

    #[arg(long, short = 's')]
    allow_any_take: Vec<Split>,

    #[arg(long, short = 'p')]
    allow_place: bool,

    #[arg(long, short = 'P')]
    pretty_print: bool,
}

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

            for split in allow_any_take {
                match split {
                    Split::Never => {
                        rule_set.push(NimRule {
                            take: TakeSize::Any,
                            split: Split::Never,
                        });
                    }
                    Split::Optional => {
                        rule_set.push(NimRule {
                            take: TakeSize::Any,
                            split: Split::Optional,
                        });
                    }
                    Split::Always => {
                        rule_set.push(NimRule {
                            take: TakeSize::Any,
                            split: Split::Always,
                        });
                    }
                }
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
