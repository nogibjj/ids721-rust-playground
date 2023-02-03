use clap::Parser;

// cargo run -- text-wrap --name "If you need a small quick library that knows how to wrap text for command line utilities, take a look at this crate."

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    TextWrap {
        #[clap(short, long)]
        name: String,
    },
}

// This is the main function
// hello::marco_polo(&name)
fn main() {
    let args = Cli::parse();
    match args.command {

        Some(Commands::TextWrap { name }) => {
            week4::txt_wrap(&name);
        }
        None => println!("No command was used"),
    }
}

