use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Zijia Jiang",
    about = "Print dataset properties"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Zijia Jiang")]
    Analyze {
        #[clap(short, long)]
        file: String,
    },
}

fn main() {
    let args = Cli::parse();
    // parse the command line arguments as the parameter of the function
    match args.command {
        Some(Commands::Analyze { file }) => {
            let contents = prj1::read_file(&file);
            println!("The dataset has the following properties: \n{}", contents);
        }
        None => {
            println!("No command given");
        }
    }
}
