// A command line tool to play Marco Polo
use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "Marco Polo",
    version = "1.0",
    author = "Ibad Rather",
    about = "A Marco Polo Game"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Ibad Rather", about = "A Marco Polo Game")]
    Play {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result = hello_marco::marco_polo(&name);
            println!("{}", result);
        }
        None => {
            println!("What's your name?");
        }
    }
}


// make run

// cargo run -- play --name "Marco" 
// cargo run -- play --name "Ibad"

// ./target/debug/hello-marco --help