use anyhow::Result;
use clap::Parser;

mod github;
mod installer;
mod market;
mod models;
mod skill_finder;

use clap::{Subcommand, ValueEnum};

use github::{DefaultFileSystem, DefaultGitHubDownloader, DefaultGitHubUrlParser};
use installer::{SkillInstaller, Target};
use market::{DefaultGitHubApiClient, FileMarketStorage, MarketService};
use skill_finder::{ConsoleUserInteraction, SkillFinder};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TargetType {
    Codex,
    Copilot,
    Claude,
}

impl Target for TargetType {
    fn as_str(&self) -> &'static str {
        match self {
            TargetType::Codex => "codex",
            TargetType::Copilot => "copilot",
            TargetType::Claude => "claude",
        }
    }
}

#[derive(Parser)]
#[command(name = "skills")]
#[command(version)]
#[command(about = "A CLI for managing skills", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install {
        #[arg(help = "Skill name or GitHub repository URL")]
        skill_or_url: String,

        #[arg(
            short = 't',
            long = "type",
            value_enum,
            help = "Target type for installation"
        )]
        target: TargetType,

        #[arg(
            short = 'g',
            long = "global",
            help = "Install globally to ~/.{type}/skills instead of ./.{type}/skills"
        )]
        global: bool,
    },
    Search {
        #[arg(help = "Search query to filter skills")]
        query: String,
    },
    Market {
        #[command(subcommand)]
        action: MarketAction,
    },
}

#[derive(Subcommand)]
enum MarketAction {
    Add {
        #[arg(
            help = "GitHub repository URL (e.g., https://github.com/owner/repo/tree/branch/path)"
        )]
        url: String,
    },
    Search {
        #[arg(help = "Search query to filter skills")]
        query: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize dependencies (Dependency Injection)
    let url_parser = DefaultGitHubUrlParser;
    let file_system = DefaultFileSystem;
    let downloader = DefaultGitHubDownloader::new(file_system);
    let storage = FileMarketStorage::new()?;
    let api_client = DefaultGitHubApiClient::new()?;
    let user_interaction = ConsoleUserInteraction;

    // Create services with injected dependencies
    let market_service = MarketService::new(storage, url_parser);
    let skill_finder = SkillFinder::new(market_service, api_client);
    let installer = SkillInstaller::new(downloader, url_parser);

    match cli.command {
        Commands::Install {
            skill_or_url,
            target,
            global,
        } => {
            if skill_or_url.starts_with("http") {
                installer.install_from_url(&skill_or_url, &target, global)?;
            } else {
                installer.install_from_market(
                    &skill_or_url,
                    &target,
                    global,
                    &skill_finder,
                    &user_interaction,
                )?;
            }
        }
        Commands::Search { query } => {
            skill_finder.search(&query)?;
        }
        Commands::Market { action } => match action {
            MarketAction::Add { url } => {
                let storage = FileMarketStorage::new()?;
                let url_parser = DefaultGitHubUrlParser;
                let market_service = MarketService::new(storage, url_parser);
                market_service.add_market(&url)?;
            }
            MarketAction::Search { query } => {
                skill_finder.search(&query)?;
            }
        },
    }

    Ok(())
}
