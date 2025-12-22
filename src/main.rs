use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand, ValueEnum};
use serde::Deserialize;
use std::fs;
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
#[command(about = "A CLI for managing skills", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install {
        #[arg(short = 't', long = "type", value_enum, help = "Target type for installation")]
        target: TargetType,

        #[arg(short = 'p', long = "path", help = "GitHub repository URL (e.g., https://github.com/owner/repo/tree/branch/path)")]
        path: String,

        #[arg(short = 'g', long = "global", help = "Install globally to ~/.{type}/skills instead of ./.{type}/skills")]
        github: bool,
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

fn search_skills(query: &str) -> Result<()> {
    let api_url = "https://api.github.com/repos/anthropics/skills/contents/skills";

    println!("Searching for skills matching '{}'...\n", query);

    let client = reqwest::blocking::Client::builder()
        .user_agent("skills-cli")
        .build()?;

    let response = client.get(api_url)
        .send()
        .context("Failed to fetch skills from GitHub")?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to fetch skills: HTTP {}", response.status()));
    }

    let contents: Vec<GitHubContent> = response.json()
        .context("Failed to parse GitHub API response")?;

    let query_lower = query.to_lowercase();
    let mut found_skills = Vec::new();

    for item in contents {
        if item.item_type == "dir" && item.name.to_lowercase().contains(&query_lower) {
            found_skills.push(item);
        }
    }

    if found_skills.is_empty() {
        println!("No skills found matching '{}'", query);
    } else {
        println!("Found {} skill(s):\n", found_skills.len());
        for skill in found_skills {
            println!("  • {} ", skill.name);
            println!("    URL: https://github.com/anthropics/skills/tree/main/{}", skill.path);
            println!();
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { target, path, github } => {
            let repo = parse_github_url(&path)
                .context("Failed to parse GitHub URL")?;

            let skill_name = extract_skill_name(&repo.path)?;
            let target_dir = get_target_directory(target, github)?;

            download_and_extract_github_folder(&repo, &target_dir, &skill_name)?;
        }
        Commands::Search { query } => {
            search_skills(&query)?;
        }
    }

    Ok(())
}
