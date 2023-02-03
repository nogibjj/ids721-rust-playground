use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Noah Gift", about = "A sort.")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift", about = "A sort.")]
    MostFreq {
        #[clap(short, long)]
        name: String,
    },
}

// This is the main function
// hello::marco_polo(&name)
fn main() {
    let args = Cli::parse();
    match args.command {

        Some(Commands::MostFreq { name }) => {
            hello::most_frequent(&name);
        }
        None => println!("No command was used"),
    }
}
// cargo run -- most-freq --name "1 4 2 1"
