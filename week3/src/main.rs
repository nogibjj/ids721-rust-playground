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
    SortMe {
        #[clap(short, long)]
        name: String,
    },
}

// This is the main function
// hello::marco_polo(&name)
fn main() {
    let args = Cli::parse();
    match args.command {

        Some(Commands::SortMe { name }) => {
            hello::my_sort(&name);
        }
        None => println!("No command was used"),
    }
}
// cargo run -- sort-me --name "1 2 4"
