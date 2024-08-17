use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "my_cli_tool")]
#[command(about = "A simple CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Create a new nv config file")]
    New {
        #[arg()]
        name: String,

        #[arg(short, long)]
        force: bool,
    },

    #[command(about = "Generate an encrypted nvx file")]
    Build {
        #[arg(short, long)]
        force: bool,
    },

    #[command(about = "Clean up your local config")]
    Burn {
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        force: bool,
    },

    #[command(about = "Push your config to the vault")]
    Push {
        #[arg(short, long)]
        force: bool,
    },

    #[command(about = "Pull a config from the vault")]
    Pull {
        #[arg(short, long)]
        force: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { force, name } => {
            println!("Creating new config with name:{} and force:{}", name, force);
        }
        Commands::Build { force } => {
            println!("Building force:{}", force);
        }
        Commands::Burn { force } => {
            println!("Burning force:{}", force);
        }
        Commands::Push { force } => {
            println!("Pushing force:{}", force);
        }
        Commands::Pull { force } => {
            println!("Pulling force:{}", force);
        }
    }
}
