use clap::Parser;

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
            println!("Splits for height {}: {:?}", height, splits);
        }
    }
}
