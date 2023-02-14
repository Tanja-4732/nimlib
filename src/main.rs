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
    #[command(about = "Calculate the nimber for a pile of given height")]
    Nimber,
    #[command(about = "Calculate all possible splits for a given height")]
    Splits {
        #[arg(help = "Height of the stack to calculate splits for")]
        height: u64,
        #[arg(short, long, help = "Output as CSV")]
        csv: bool,
    },
}

pub fn main() {
    let args = Cli::parse();
    match args.action {
        Action::Nimber => {
            // Action::CalculateNimber(h) => {
            // let nimber = nimlib::nimbers::calculate_nimber_for_height(h);
            println!("")
        }
        Action::Splits { height, csv } => {
            let splits = nimlib::nimbers::calculate_splits(height);

            if csv {
                println!("left,right");
                for (Stack(left), Stack(right)) in splits {
                    println!("{},{}", left, right);
                }
                return;
            }

            if splits.len() == 0 {
                println!("No splits for height {}", height);
                return;
            }

            println!("Splits for height {height}:");

            let max_digits_left = splits[splits.len() - 1].0 .0.ilog10() as usize + 1;
            let max_digits_right = splits[0].1 .0.ilog10() as usize + 1;

            for (Stack(left), Stack(right)) in splits {
                println!("{left:max_digits_left$} + {right:max_digits_right$}");
            }
        }
    }
}
