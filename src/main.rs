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

#![deny(missing_docs, clippy::missing_docs_in_private_items)]

use std::ops::ControlFlow;

use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::Verbosity;
use log::LevelFilter;
use nimlib::{nimbers, NimRule, Nimber, Split, Stack, TakeSize};
use serde::Serialize;

#[derive(clap::Parser)]
#[command(
    about = "A Rust CLI tool for Nim games: calculate nimbers and possible moves",
    version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS")
)]
#[derive(Debug)]
struct Cli {
    #[command(subcommand)]
    action: Action,

    /// Verbosity level (-v, -vv, -vvv, etc.)
    #[command(flatten)]
    verbose: Verbosity,
}

#[derive(Subcommand, Debug)]
enum Action {
    #[command(about = "Calculate the nimber for a stack of given height")]
    Nimber {
        /// The heights of the stacks of a position to calculate the nimber for
        #[arg()]
        heights: Vec<u64>,

        // #[arg(long, short = 'r', help = "Use the rules from the given JSON file")]
        // rules_file: Option<String>,
        /// A JSON string containing the rules to use for the calculation (see `nimlib make-rule-set`)
        #[arg(long, short)]
        rules: String,

        // #[arg(long, short = 'c', help = "Number of pool coins")]
        // pool_coins: u64,
        /// Print either the nimbers of the stacks, of the entire position, or both
        #[arg(long, short)]
        print: Option<PrintNimbers>,

        #[arg(long, short)]
        /// Print the result as JSON
        json: bool,

        #[arg(long, short = 'J')]
        /// Pretty-print the JSON output
        json_pretty: bool,
    },
    #[command(about = "Calculate all possible splits for a given height")]
    Splits {
        /// Height of the stack to calculate splits for
        #[arg()]
        height: u64,

        /// Print the splits as a CSV (with commas, newlines, and a header)
        #[arg(short, long)]
        csv: bool,
    },
    #[command(about = "Create a JSON rule set using CLI parameters")]
    MakeRuleSet(MakeRuleSet),
}

#[derive(ValueEnum, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PrintNimbers {
    Stacks,
    Position,
    #[default]
    Both,
}

#[derive(Args, Debug, Serialize)]
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
    #[arg(long, short = 'A')]
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

    // Set the log level
    env_logger::builder()
        .filter_level(
            args.verbose
                .log_level()
                .map(|v| v.to_level_filter())
                .unwrap_or(log::LevelFilter::Warn),
        )
        .init();

    log::info!("nimlib version {}", env!("CARGO_PKG_VERSION"));

    log::trace!("CLI arguments: {:#?}", &args);
    log::trace!("Log level: {:#?}", &args.verbose.log_level());

    match args.action {
        Action::Nimber {
            heights,
            rules,
            print: print_style,
            json,
            json_pretty,
        } => calculate_nimbers(print_style, rules, heights, json, json_pretty),
        Action::Splits { height, csv } => calculate_splits(height, csv),
        Action::MakeRuleSet(options) => make_rule_set(options),
    }
}

fn make_rule_set(
    MakeRuleSet {
        take_split_never,
        take_split_optional,
        take_split_always,
        allow_any_take,
        allow_place,
        pretty_print,
    }: MakeRuleSet,
) {
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
    let rules = if pretty_print {
        serde_json::to_string_pretty(&rule_set).unwrap()
    } else {
        serde_json::to_string(&rule_set).unwrap()
    };
    println!("{rules}");
}

fn calculate_splits(height: u64, csv: bool) {
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

fn calculate_nimbers(
    print_style: Option<PrintNimbers>,
    rules: String,
    heights: Vec<u64>,
    json: bool,
    json_pretty: bool,
) {
    let print_style = print_style.unwrap_or_default();
    let rules: Vec<NimRule> = serde_json::from_str(&rules).unwrap();
    let mut nimbers = Vec::new();
    for height in heights {
        let nimber = nimbers::calculate_nimber_for_height(height, &rules, 0);
        if print_style != PrintNimbers::Position && !json && !json_pretty {
            println!("Nimber for stack of height {height}: {nimber}");
        }
        nimbers.push(nimber);
    }
    let nimber = Nimber(nimbers.iter().fold(0, |acc, x| acc ^ x.0));
    if nimbers.len() > 1 && print_style != PrintNimbers::Stacks && !json && !json_pretty {
        println!("Nimber for the position: {nimber}");
    }
    #[derive(Serialize)]
    struct Result {
        stack_nimbers: Vec<Nimber>,
        position_nimber: Nimber,
    }
    if json_pretty {
        let json = match print_style {
            PrintNimbers::Stacks => serde_json::to_string_pretty(&nimbers).unwrap(),
            PrintNimbers::Position => serde_json::to_string_pretty(&nimber).unwrap(),
            PrintNimbers::Both => serde_json::to_string_pretty(&Result {
                stack_nimbers: nimbers,
                position_nimber: nimber,
            })
            .unwrap(),
        };

        println!("{json}");
    } else if json {
        let json = match print_style {
            PrintNimbers::Stacks => serde_json::to_string(&nimbers).unwrap(),
            PrintNimbers::Position => serde_json::to_string(&nimber).unwrap(),
            PrintNimbers::Both => serde_json::to_string(&Result {
                stack_nimbers: nimbers,
                position_nimber: nimber,
            })
            .unwrap(),
        };

        println!("{json}");
    }

    if json && json_pretty {
        log::warn!("--json and --json-pretty are mutually exclusive. Ignoring --json.");
    }
}
