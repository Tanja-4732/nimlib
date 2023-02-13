use clap::Parser;
use nimlib::Stack;

#[derive(clap::Parser)]
#[command(about = "A Nim-game CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {
    Nimber,
    Splits { height: u64 },
}

pub fn main() {
    let args = Cli::parse();
    match args.action {
        Action::Nimber => {
            // Action::CalculateNimber(h) => {
            // let nimber = nimlib::nimbers::calculate_nimber_for_height(h);
            println!("")
        }
        Action::Splits { height } => {
            let splits = nimlib::nimbers::calculate_splits(height);
            println!("Splits for height {height}:");

            // TODO optimize this; avoid creating strings
            let max_digits_left = splits[splits.len() - 1].0 .0.to_string().len();
            let max_digits_right = splits[0].1 .0.to_string().len();

            for (Stack(left), Stack(right)) in splits {
                println!("{left:max_digits_left$} + {right:max_digits_right$}");
            }
        }
    }
}
