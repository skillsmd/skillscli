use anyhow::{Context, Result, anyhow};
use std::path::PathBuf;

use crate::github::{GitHubDownloader, GitHubUrlParser, extract_skill_name};
use crate::market::{GitHubApiClient, MarketStorage};
use crate::models::SkillMatch;
use crate::skill_finder::{SkillFinder, UserInteraction};

/// Trait for target type abstraction
pub trait Target {
    fn as_str(&self) -> &'static str;
}

/// Service for installing skills
pub struct SkillInstaller<D: GitHubDownloader, P: GitHubUrlParser> {
    downloader: D,
    url_parser: P,
}

impl<D: GitHubDownloader, P: GitHubUrlParser> SkillInstaller<D, P> {
    pub fn new(downloader: D, url_parser: P) -> Self {
        Self {
            downloader,
            url_parser,
        }
    }

    pub fn install_from_url<T: Target>(&self, url: &str, target: &T, global: bool) -> Result<()> {
        let repo = self
            .url_parser
            .parse(url)
            .context("Failed to parse GitHub URL")?;
        let skill_name = extract_skill_name(&repo.path)?;
        let target_dir = get_target_directory(target, global)?;

        self.downloader
            .download_folder(&repo, &target_dir, &skill_name)?;

        Ok(())
    }

    pub fn install_from_market<S, U, A, I, T>(
        &self,
        skill_name: &str,
        target: &T,
        global: bool,
        skill_finder: &SkillFinder<S, U, A>,
        user_interaction: &I,
    ) -> Result<()>
    where
        S: MarketStorage,
        U: GitHubUrlParser,
        A: GitHubApiClient,
        I: UserInteraction,
        T: Target,
    {
        println!("Searching for skill '{}' in markets...\n", skill_name);
        let matches = skill_finder.find_by_name(skill_name)?;

        if matches.is_empty() {
            return Err(anyhow!(
                "No available skill '{}' in the market. Please add the market first using 'skills market add <url>'",
                skill_name
            ));
        }

        let selected = self.select_skill(&matches, user_interaction)?;

        println!(
            "Installing {} from {}...\n",
            selected.name, selected.market_name
        );

        let repo = self
            .url_parser
            .parse(&selected.url)
            .context("Failed to parse skill URL")?;
        let target_dir = get_target_directory(target, global)?;

        self.downloader
            .download_folder(&repo, &target_dir, &selected.name)?;

        Ok(())
    }

    fn select_skill<'a, I: UserInteraction>(
        &self,
        matches: &'a [SkillMatch],
        user_interaction: &I,
    ) -> Result<&'a SkillMatch> {
        if matches.len() == 1 {
            println!(
                "Found skill: {} ({})",
                matches[0].name, matches[0].market_name
            );
            Ok(&matches[0])
        } else {
            user_interaction.select_skill(matches)
        }
    }
}

fn get_target_directory<T: Target>(target: &T, global: bool) -> Result<PathBuf> {
    let base_dir = if global {
        dirs::home_dir().ok_or_else(|| anyhow!("Could not determine home directory"))?
    } else {
        std::env::current_dir().context("Failed to get current directory")?
    };

    let folder_name = if target.as_str() == "copilot" {
        ".github".to_string()
    } else {
        format!(".{}", target.as_str())
    };

    Ok(base_dir.join(folder_name).join("skills"))
}
