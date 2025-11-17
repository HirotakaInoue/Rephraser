use clap::Parser;
use rephraser::cli::{Cli, Commands, ConfigCommands};
use rephraser::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Rephrase { action, text } => {
            rephraser::cli::commands::rephrase(&action, &text).await?;
        }
        Commands::ListActions => {
            rephraser::cli::commands::list_actions().await?;
        }
        Commands::Config { subcommand } => match subcommand {
            ConfigCommands::Init => {
                rephraser::cli::commands::config_init().await?;
            }
            ConfigCommands::Show => {
                rephraser::cli::commands::config_show().await?;
            }
            ConfigCommands::Set { key, value } => {
                rephraser::cli::commands::config_set(&key, &value).await?;
            }
            ConfigCommands::Path => {
                rephraser::cli::commands::config_path().await?;
            }
        },
    }

    Ok(())
}
