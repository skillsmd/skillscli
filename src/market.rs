use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::github::GitHubUrlParser;
use crate::models::{GitHubContent, MarketEntry};

/// Trait for accessing market configuration storage
pub trait MarketStorage {
    fn load(&self) -> Result<Vec<MarketEntry>>;
    fn save(&self, markets: &[MarketEntry]) -> Result<()>;
}

/// Trait for interacting with GitHub API
pub trait GitHubApiClient {
    fn get_directory_contents(&self, repo: &str, path: &str) -> Result<Vec<GitHubContent>>;
}

/// Default implementation of MarketStorage using file system
pub struct FileMarketStorage {
    config_path: PathBuf,
}

impl FileMarketStorage {
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow!("Could not determine home directory"))?;
        let config_path = home_dir.join(".skills").join("market.json");
        Ok(Self { config_path })
    }
}

impl MarketStorage for FileMarketStorage {
    fn load(&self) -> Result<Vec<MarketEntry>> {
        if !self.config_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.config_path)
            .context("Failed to read market.json")?;

        let markets: Vec<MarketEntry> = serde_json::from_str(&content)
            .context("Failed to parse market.json")?;

        Ok(markets)
    }

    fn save(&self, markets: &[MarketEntry]) -> Result<()> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create .skills directory")?;
        }

        let json = serde_json::to_string_pretty(markets)
            .context("Failed to serialize markets")?;

        fs::write(&self.config_path, json)
            .context("Failed to write market.json")?;

        Ok(())
    }
}

/// Default implementation of GitHubApiClient
pub struct DefaultGitHubApiClient {
    client: reqwest::blocking::Client,
}

impl DefaultGitHubApiClient {
    pub fn new() -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("skills-cli")
            .build()?;
        Ok(Self { client })
    }
}

impl GitHubApiClient for DefaultGitHubApiClient {
    fn get_directory_contents(&self, repo: &str, path: &str) -> Result<Vec<GitHubContent>> {
        let api_url = format!("https://api.github.com/repos/{}/contents/{}", repo, path);

        let response = self
            .client
            .get(&api_url)
            .send()
            .context(format!("Failed to fetch from {}", repo))?;

        if !response.status().is_success() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let contents: Vec<GitHubContent> = response
            .json()
            .context("Failed to parse GitHub API response")?;

        Ok(contents)
    }
}

/// Service for managing markets
pub struct MarketService<S: MarketStorage, U: GitHubUrlParser> {
    storage: S,
    url_parser: U,
}

impl<S: MarketStorage, U: GitHubUrlParser> MarketService<S, U> {
    pub fn new(storage: S, url_parser: U) -> Self {
        Self { storage, url_parser }
    }

    pub fn add_market(&self, url: &str) -> Result<()> {
        let mut markets = self.storage.load()?;

        let name = self.extract_repo_name(url)?;

        if markets.iter().any(|m| m.url == url) {
            println!("Market '{}' is already added", name);
            return Ok(());
        }

        markets.push(MarketEntry {
            name,
            url: url.to_string(),
        });

        self.storage.save(&markets)?;

        println!("Successfully added market: {}", url);
        Ok(())
    }

    pub fn get_repositories(&self) -> Result<Vec<(String, String, String, String)>> {
        let mut repositories = vec![(
            "anthropics/skills".to_string(),
            "skills".to_string(),
            "https://github.com/anthropics/skills/tree/main".to_string(),
            "anthropics/skills".to_string(),
        )];

        let markets = self.storage.load()?;
        for market in markets {
            let parsed = self.url_parser.parse(&market.url)?;
            let repo_path = format!("{}/{}", parsed.owner, parsed.repo);
            let base_url = format!(
                "https://github.com/{}/tree/{}",
                repo_path, parsed.branch
            );

            let repo_key = format!("{}/{}", repo_path, parsed.path);
            let is_duplicate = repositories
                .iter()
                .any(|(r, p, _, _)| format!("{}/{}", r, p) == repo_key);

            if !is_duplicate {
                repositories.push((repo_path, parsed.path, base_url, market.name.clone()));
            }
        }

        Ok(repositories)
    }

    fn extract_repo_name(&self, url: &str) -> Result<String> {
        let parsed = self.url_parser.parse(url)?;
        Ok(format!("{}/{}", parsed.owner, parsed.repo))
    }
}
