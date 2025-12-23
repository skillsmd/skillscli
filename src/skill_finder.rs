use anyhow::Result;
use std::io::{self, Write};

use crate::github::GitHubUrlParser;
use crate::market::{GitHubApiClient, MarketService, MarketStorage};
use crate::models::{GitHubContent, SkillMatch};

/// Service for finding and searching skills
pub struct SkillFinder<S: MarketStorage, U: GitHubUrlParser, A: GitHubApiClient> {
    market_service: MarketService<S, U>,
    api_client: A,
}

impl<S: MarketStorage, U: GitHubUrlParser, A: GitHubApiClient> SkillFinder<S, U, A> {
    pub fn new(market_service: MarketService<S, U>, api_client: A) -> Self {
        Self {
            market_service,
            api_client,
        }
    }

    pub fn find_by_name(&self, skill_name: &str) -> Result<Vec<SkillMatch>> {
        let repositories = self.market_service.get_repositories()?;

        if repositories.is_empty() {
            return Ok(Vec::new());
        }

        let skill_name_lower = skill_name.to_lowercase();
        let mut matches = Vec::new();

        for (repo, path, base_url, market_name) in repositories {
            let contents = match self.api_client.get_directory_contents(&repo, &path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            for item in contents {
                if item.item_type == "dir" && item.name.to_lowercase() == skill_name_lower {
                    matches.push(SkillMatch {
                        name: item.name.clone(),
                        url: format!("{}/{}", base_url, item.path),
                        market_name: market_name.clone(),
                    });
                }
            }
        }

        Ok(matches)
    }

    pub fn search(&self, query: &str) -> Result<()> {
        let repositories = self.market_service.get_repositories()?;

        println!("Searching for skills matching '{}'...\n", query);

        let query_lower = query.to_lowercase();
        let mut all_found_skills = Vec::new();

        for (repo, path, base_url, market_name) in repositories {
            let contents = match self.api_client.get_directory_contents(&repo, &path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Warning: Failed to fetch from {}: {}", repo, e);
                    continue;
                }
            };

            for item in contents {
                if item.item_type == "dir" && item.name.to_lowercase().contains(&query_lower) {
                    all_found_skills.push((item, base_url.clone(), market_name.clone()));
                }
            }
        }

        self.display_search_results(&all_found_skills, query);

        Ok(())
    }

    fn display_search_results(&self, results: &[(GitHubContent, String, String)], query: &str) {
        if results.is_empty() {
            println!("No skills found matching '{}'", query);
        } else {
            println!("Found {} skill(s):\n", results.len());
            for (skill, base_url, market_name) in results {
                println!("  • {} ({})", skill.name, market_name);
                println!("    URL: {}/{}", base_url, skill.path);
                println!();
            }
        }
    }
}

/// Trait for user interaction
pub trait UserInteraction {
    fn select_skill<'a>(&self, matches: &'a [SkillMatch]) -> Result<&'a SkillMatch>;
}

/// Console-based user interaction
pub struct ConsoleUserInteraction;

impl UserInteraction for ConsoleUserInteraction {
    fn select_skill<'a>(&self, matches: &'a [SkillMatch]) -> Result<&'a SkillMatch> {
        println!("Multiple skills found. Please select one:");
        for (i, skill) in matches.iter().enumerate() {
            println!("  {}. {} ({})", i + 1, skill.name, skill.market_name);
        }

        print!("\nEnter your choice (1-{}): ", matches.len());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let choice: usize = input
            .trim()
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid input, please enter a number"))?;

        if choice < 1 || choice > matches.len() {
            return Err(anyhow::anyhow!(
                "Invalid choice, must be between 1 and {}",
                matches.len()
            ));
        }

        Ok(&matches[choice - 1])
    }
}
