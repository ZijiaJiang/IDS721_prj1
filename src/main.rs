use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Zijia Jiang",
    about = "Search for the most related name."
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Zijia Jiang")]
    Search {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    // parse the command line arguments as the parameters of the function
    match args.command {
        Some(Commands::Search { name }) => {
            let result = prj1::search_name(name);
            println!(
                "The person who is most related with the name you search is {}",
                result
            );
        }
        None => {
            println!("Please input the name you want to search.");
        }
    }
}
