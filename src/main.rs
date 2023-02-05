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
            let columns = prj1::read_csv(&file);
            println!(
                "There are {} features in this dataset.\nThe information are as follows:",
                columns.len()
            );
            prj1::print_vec_str(columns);
            println!("Please use the information as an reference to choose the dataset that best matches your needs.")
        }
        None => {
            println!("No command given");
        }
    }
}
