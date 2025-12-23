use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum TargetType {
    Codex,
    Copilot,
}

impl TargetType {
    fn as_str(&self) -> &'static str {
        match self {
            TargetType::Codex => "codex",
            TargetType::Copilot => "copilot",
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

        #[arg(short = 't', long = "type", value_enum, help = "Target type for installation")]
        target: TargetType,

        #[arg(short = 'g', long = "global", help = "Install globally to ~/.{type}/skills instead of ./.{type}/skills")]
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
        #[arg(help = "GitHub repository URL (e.g., https://github.com/owner/repo/tree/branch/path)")]
        url: String,
    },
    Search {
        #[arg(help = "Search query to filter skills")]
        query: String,
    },
}

#[derive(Debug)]
struct GitHubRepo {
    owner: String,
    repo: String,
    branch: String,
    path: String,
}

#[derive(Debug, Deserialize)]
struct GitHubContent {
    name: String,
    #[serde(rename = "type")]
    item_type: String,
    path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MarketEntry {
    name: String,
    url: String,
}

#[derive(Debug, Clone)]
struct SkillMatch {
    name: String,
    url: String,
    market_name: String,
}

fn parse_github_url(url: &str) -> Result<GitHubRepo> {
    let url = url.trim_end_matches('/');

    let parts: Vec<&str> = url.split('/').collect();

    if parts.len() < 5 || !url.contains("github.com") {
        return Err(anyhow!("Invalid GitHub URL format"));
    }

    let github_index = parts.iter().position(|&x| x == "github.com")
        .ok_or_else(|| anyhow!("github.com not found in URL"))?;

    let owner = parts.get(github_index + 1)
        .ok_or_else(|| anyhow!("Owner not found in URL"))?;
    let repo = parts.get(github_index + 2)
        .ok_or_else(|| anyhow!("Repo not found in URL"))?;

    let tree_index = parts.iter().position(|&x| x == "tree");

    let (branch, path) = if let Some(idx) = tree_index {
        let branch = parts.get(idx + 1)
            .ok_or_else(|| anyhow!("Branch not found in URL"))?;
        let path = parts[idx + 2..].join("/");
        (*branch, path)
    } else {
        ("main", String::new())
    };

    Ok(GitHubRepo {
        owner: owner.to_string(),
        repo: repo.to_string(),
        branch: branch.to_string(),
        path,
    })
}

fn download_and_extract_github_folder(
    repo: &GitHubRepo,
    target_dir: &Path,
    skill_name: &str,
) -> Result<()> {
    let zip_url = format!(
        "https://github.com/{}/{}/archive/refs/heads/{}.zip",
        repo.owner, repo.repo, repo.branch
    );

    println!("Downloading from GitHub: {}", zip_url);

    let response = reqwest::blocking::get(&zip_url)
        .context("Failed to download repository")?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to download: HTTP {}", response.status()));
    }

    let temp_dir = tempfile::tempdir().context("Failed to create temp directory")?;
    let zip_path = temp_dir.path().join("repo.zip");

    let bytes = response.bytes().context("Failed to read response bytes")?;
    fs::write(&zip_path, bytes).context("Failed to write zip file")?;

    let file = fs::File::open(&zip_path).context("Failed to open zip file")?;
    let mut archive = zip::ZipArchive::new(file).context("Failed to read zip archive")?;

    let extract_dir = temp_dir.path().join("extracted");
    fs::create_dir_all(&extract_dir).context("Failed to create extraction directory")?;

    archive.extract(&extract_dir).context("Failed to extract archive")?;

    let source_path = if repo.path.is_empty() {
        extract_dir.join(format!("{}-{}", repo.repo, repo.branch))
    } else {
        extract_dir
            .join(format!("{}-{}", repo.repo, repo.branch))
            .join(&repo.path)
    };

    if !source_path.exists() {
        return Err(anyhow!(
            "Path '{}' not found in repository",
            repo.path
        ));
    }

    let dest_path = target_dir.join(skill_name);
    fs::create_dir_all(&dest_path)
        .context("Failed to create destination directory")?;

    println!("Copying files to: {}", dest_path.display());
    copy_dir_all(&source_path, &dest_path)?;

    println!("Successfully installed skill to: {}", dest_path.display());

    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in WalkDir::new(src).min_depth(1) {
        let entry = entry?;
        let path = entry.path();

        let relative_path = path.strip_prefix(src)
            .context("Failed to get relative path")?;
        let dest_path = dst.join(relative_path);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &dest_path)?;
        }
    }

    Ok(())
}

fn get_target_directory(target: TargetType, global: bool) -> Result<PathBuf> {
    let base_dir = if global {
        dirs::home_dir()
            .ok_or_else(|| anyhow!("Could not determine home directory"))?
    } else {
        std::env::current_dir()
            .context("Failed to get current directory")?
    };

    Ok(base_dir.join(format!(".{}", target.as_str())).join("skills"))
}

fn extract_skill_name(path: &str) -> Result<String> {
    let path = path.trim_end_matches('/');
    let name = path.split('/').last()
        .ok_or_else(|| anyhow!("Could not extract skill name from path"))?;
    Ok(name.to_string())
}

fn get_market_config_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not determine home directory"))?;
    Ok(home_dir.join(".skills").join("market.json"))
}

fn load_markets() -> Result<Vec<MarketEntry>> {
    let config_path = get_market_config_path()?;

    if !config_path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&config_path)
        .context("Failed to read market.json")?;

    let markets: Vec<MarketEntry> = serde_json::from_str(&content)
        .context("Failed to parse market.json")?;

    Ok(markets)
}

fn save_markets(markets: &[MarketEntry]) -> Result<()> {
    let config_path = get_market_config_path()?;

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create .skills directory")?;
    }

    let json = serde_json::to_string_pretty(markets)
        .context("Failed to serialize markets")?;

    fs::write(&config_path, json)
        .context("Failed to write market.json")?;

    Ok(())
}

fn extract_repo_name_from_url(url: &str) -> Result<String> {
    let parsed = parse_github_url(url)?;
    Ok(format!("{}/{}", parsed.owner, parsed.repo))
}

fn add_market(url: &str) -> Result<()> {
    let mut markets = load_markets()?;

    let name = extract_repo_name_from_url(url)?;

    // Check if market already exists
    if markets.iter().any(|m| m.url == url) {
        println!("Market '{}' is already added", name);
        return Ok(());
    }

    markets.push(MarketEntry {
        name,
        url: url.to_string(),
    });

    save_markets(&markets)?;

    println!("Successfully added market: {}", url);
    Ok(())
}

fn get_market_repositories() -> Result<Vec<(String, String, String, String)>> {
    let mut repositories = vec![
        ("anthropics/skills".to_string(), "skills".to_string(), "https://github.com/anthropics/skills/tree/main".to_string(), "anthropics/skills".to_string()),
    ];

    // Load custom markets from config
    let markets = load_markets()?;
    for market in markets {
        let parsed = parse_github_url(&market.url)?;
        let repo_path = format!("{}/{}", parsed.owner, parsed.repo);
        let base_url = format!("https://github.com/{}/tree/{}", repo_path, parsed.branch);

        // Check if this repository is already in the list (deduplicate)
        let repo_key = format!("{}/{}", repo_path, parsed.path);
        let is_duplicate = repositories.iter().any(|(r, p, _, _)| {
            format!("{}/{}", r, p) == repo_key
        });

        if !is_duplicate {
            repositories.push((repo_path, parsed.path, base_url, market.name.clone()));
        }
    }

    Ok(repositories)
}

fn find_skills_by_name(skill_name: &str) -> Result<Vec<SkillMatch>> {
    let repositories = get_market_repositories()?;

    if repositories.is_empty() {
        return Ok(Vec::new());
    }

    let client = reqwest::blocking::Client::builder()
        .user_agent("skills-cli")
        .build()?;

    let skill_name_lower = skill_name.to_lowercase();
    let mut matches = Vec::new();

    for (repo, path, base_url, market_name) in repositories {
        let api_url = format!("https://api.github.com/repos/{}/contents/{}", repo, path);

        let response = match client.get(&api_url).send() {
            Ok(r) => r,
            Err(_) => continue,
        };

        if !response.status().is_success() {
            continue;
        }

        let contents: Vec<GitHubContent> = match response.json() {
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

fn select_skill(matches: &[SkillMatch]) -> Result<&SkillMatch> {
    println!("Multiple skills found. Please select one:");
    for (i, skill) in matches.iter().enumerate() {
        println!("  {}. {} ({})", i + 1, skill.name, skill.market_name);
    }

    print!("\nEnter your choice (1-{}): ", matches.len());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let choice: usize = input.trim().parse()
        .context("Invalid input, please enter a number")?;

    if choice < 1 || choice > matches.len() {
        return Err(anyhow!("Invalid choice, must be between 1 and {}", matches.len()));
    }

    Ok(&matches[choice - 1])
}

fn search_skills(query: &str) -> Result<()> {
    let repositories = get_market_repositories()?;

    println!("Searching for skills matching '{}'...\n", query);

    let client = reqwest::blocking::Client::builder()
        .user_agent("skills-cli")
        .build()?;

    let query_lower = query.to_lowercase();
    let mut all_found_skills = Vec::new();

    for (repo, path, base_url, market_name) in repositories {
        let api_url = format!("https://api.github.com/repos/{}/contents/{}", repo, path);

        let response = client.get(&api_url)
            .send()
            .context(format!("Failed to fetch skills from {}", repo))?;

        if !response.status().is_success() {
            eprintln!("Warning: Failed to fetch from {}: HTTP {}", repo, response.status());
            continue;
        }

        let contents: Vec<GitHubContent> = match response.json() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Warning: Failed to parse response from {}: {}", repo, e);
                continue;
            }
        };

        for item in contents {
            if item.item_type == "dir" && item.name.to_lowercase().contains(&query_lower) {
                all_found_skills.push((item, base_url.clone(), market_name.clone()));
            }
        }
    }

    if all_found_skills.is_empty() {
        println!("No skills found matching '{}'", query);
    } else {
        println!("Found {} skill(s):\n", all_found_skills.len());
        for (skill, base_url, market_name) in all_found_skills {
            println!("  • {} ({})", skill.name, market_name);
            println!("    URL: {}/{}", base_url, skill.path);
            println!();
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { skill_or_url, target, global } => {
            let (repo, skill_name) = if skill_or_url.starts_with("http") {
                // URL provided - use traditional install
                let repo = parse_github_url(&skill_or_url)
                    .context("Failed to parse GitHub URL")?;
                let skill_name = extract_skill_name(&repo.path)?;
                (repo, skill_name)
            } else {
                // Skill name provided - search in markets
                println!("Searching for skill '{}' in markets...\n", skill_or_url);
                let matches = find_skills_by_name(&skill_or_url)?;

                if matches.is_empty() {
                    return Err(anyhow!(
                        "No available skill '{}' in the market. Please add the market first using 'skills market add <url>'",
                        skill_or_url
                    ));
                }

                let selected = if matches.len() == 1 {
                    println!("Found skill: {} ({})", matches[0].name, matches[0].market_name);
                    &matches[0]
                } else {
                    select_skill(&matches)?
                };

                println!("Installing {} from {}...\n", selected.name, selected.market_name);
                let repo = parse_github_url(&selected.url)
                    .context("Failed to parse skill URL")?;
                (repo, selected.name.clone())
            };

            let target_dir = get_target_directory(target, global)?;
            download_and_extract_github_folder(&repo, &target_dir, &skill_name)?;
        }
        Commands::Search { query } => {
            search_skills(&query)?;
        }
        Commands::Market { action } => {
            match action {
                MarketAction::Add { url } => {
                    add_market(&url)?;
                }
                MarketAction::Search { query } => {
                    search_skills(&query)?;
                }
            }
        }
    }

    Ok(())
}
