mod commands;

use clap::{Parser, Subcommand};
use commands::{
    build::{build, BuildOpts},
    dev::{lex::lex, parse::parse, resolve::resolve},
};

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
        #[arg()]
        target_config: String,

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

    #[command(about = "Commands used for development purposes")]
    Dev {
        #[command(subcommand)]
        subcommand: DevSubcommands,
    },
}

#[derive(Subcommand, Debug)]
enum DevSubcommands {
    #[command(about = "Lex the tokens of a config file")]
    Lex {
        #[arg()]
        target_config: String,
    },

    #[command(about = "Parse the AST of a config file")]
    Parse {
        #[arg()]
        target_config: String,
    },

    #[command(about = "Resolve the values of a config file")]
    Resolve {
        #[arg()]
        target_config: String,
    },
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::New { force, name } => {
            println!("Creating new config with name:{} and force:{}", name, force);
        }
        Commands::Build {
            force,
            target_config,
        } => build(BuildOpts {
            force,
            target_config,
        })
        .await
        .expect("Failed to build"),
        Commands::Burn { force } => {
            println!("Burning force:{}", force);
        }
        Commands::Push { force } => {
            println!("Pushing force:{}", force);
        }
        Commands::Pull { force } => {
            println!("Pulling force:{}", force);
        }
        Commands::Dev { subcommand } => match subcommand {
            DevSubcommands::Lex { target_config } => lex(target_config),
            DevSubcommands::Parse { target_config } => parse(target_config),
            DevSubcommands::Resolve { target_config } => {
                let _ = resolve(target_config).await.expect("Failed to resolve");
            }
        },
    }
}
