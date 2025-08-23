use anyhow::{Result, anyhow};
use chat_bot::{chat::ChatSession, client::OpenAIClient, config::Config, tool::ToolSet};
use clap::{Parser, Subcommand};
use std::sync::Arc;

#[derive(Parser)]
#[command(version, author, about = "Custom Chat Bot")]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Output Default Config
    Config,

    // Start Chat
    Chat {
        #[arg(short, long)]
        model: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config => {
            println!("{}", include_str!("config.toml"))
        }
        Commands::Chat { model } => {
            let mut config = match cli.config {
                Some(path) => Config::load(path).await?,
                None => return Err(anyhow!("Config not set")),
            };

            if let Some(mode_name) = model {
                config.model_name = Some(mode_name);
            }

            let api_key = config
                .openai_key
                .clone()
                .unwrap_or_else(|| std::env::var("OPENAI_API_KEY").expect("key must be set"));

            let url = config.chat_url.clone();
            println!("Using url {:?}", url);

            let open_ai_client = Arc::new(OpenAIClient::new(api_key, url));

            let support_tool = config.support_tool.unwrap_or(true);

            let system_prompt = if support_tool {
                "you are a assistant, you can help user to complete various tasks.".to_string()
            } else {
                "you are an assistant, you can help user to complete various tasks.".to_string()
            };

            let mut session = ChatSession::new(
                open_ai_client,
                ToolSet::default(),
                config
                    .model_name
                    .unwrap_or_else(|| "gpt-4o-mini".to_string()),
            );

            session.add_system_prompt(system_prompt);
            session.chat(support_tool).await?;
        }
    }

    Ok(())
}
