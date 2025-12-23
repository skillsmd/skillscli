use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;
use tempfile::TempDir;
use walkdir::WalkDir;

use crate::models::GitHubRepo;

/// Trait for parsing GitHub URLs
pub trait GitHubUrlParser {
    fn parse(&self, url: &str) -> Result<GitHubRepo>;
}

/// Trait for downloading content from GitHub
pub trait GitHubDownloader {
    fn download_folder(&self, repo: &GitHubRepo, target_dir: &Path, skill_name: &str) -> Result<()>;
}

/// Trait for file system operations
pub trait FileSystem {
    fn copy_dir_all(&self, src: &Path, dst: &Path) -> Result<()>;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn write_file(&self, path: &Path, content: &[u8]) -> Result<()>;
}

/// Default implementation of GitHubUrlParser
#[derive(Clone, Copy)]
pub struct DefaultGitHubUrlParser;

impl GitHubUrlParser for DefaultGitHubUrlParser {
    fn parse(&self, url: &str) -> Result<GitHubRepo> {
        let url = url.trim_end_matches('/');
        let parts: Vec<&str> = url.split('/').collect();

        if parts.len() < 5 || !url.contains("github.com") {
            return Err(anyhow!("Invalid GitHub URL format"));
        }

        let github_index = parts
            .iter()
            .position(|&x| x == "github.com")
            .ok_or_else(|| anyhow!("github.com not found in URL"))?;

        let owner = parts
            .get(github_index + 1)
            .ok_or_else(|| anyhow!("Owner not found in URL"))?;
        let repo = parts
            .get(github_index + 2)
            .ok_or_else(|| anyhow!("Repo not found in URL"))?;

        let tree_index = parts.iter().position(|&x| x == "tree");

        let (branch, path) = if let Some(idx) = tree_index {
            let branch = parts
                .get(idx + 1)
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
}

/// Default implementation of FileSystem
#[derive(Clone, Copy)]
pub struct DefaultFileSystem;

impl FileSystem for DefaultFileSystem {
    fn copy_dir_all(&self, src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;

        for entry in WalkDir::new(src).min_depth(1) {
            let entry = entry?;
            let path = entry.path();

            let relative_path = path
                .strip_prefix(src)
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

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).context("Failed to create directory")
    }

    fn write_file(&self, path: &Path, content: &[u8]) -> Result<()> {
        fs::write(path, content).context("Failed to write file")
    }
}

/// Default implementation of GitHubDownloader
pub struct DefaultGitHubDownloader<F: FileSystem> {
    file_system: F,
}

impl<F: FileSystem> DefaultGitHubDownloader<F> {
    pub fn new(file_system: F) -> Self {
        Self { file_system }
    }
}

impl<F: FileSystem> GitHubDownloader for DefaultGitHubDownloader<F> {
    fn download_folder(&self, repo: &GitHubRepo, target_dir: &Path, skill_name: &str) -> Result<()> {
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

        let temp_dir = TempDir::new().context("Failed to create temp directory")?;
        let zip_path = temp_dir.path().join("repo.zip");

        let bytes = response.bytes().context("Failed to read response bytes")?;
        self.file_system.write_file(&zip_path, &bytes)?;

        let file = fs::File::open(&zip_path).context("Failed to open zip file")?;
        let mut archive = zip::ZipArchive::new(file).context("Failed to read zip archive")?;

        let extract_dir = temp_dir.path().join("extracted");
        self.file_system.create_dir_all(&extract_dir)?;

        archive
            .extract(&extract_dir)
            .context("Failed to extract archive")?;

        let source_path = if repo.path.is_empty() {
            extract_dir.join(format!("{}-{}", repo.repo, repo.branch))
        } else {
            extract_dir
                .join(format!("{}-{}", repo.repo, repo.branch))
                .join(&repo.path)
        };

        if !source_path.exists() {
            return Err(anyhow!("Path '{}' not found in repository", repo.path));
        }

        let dest_path = target_dir.join(skill_name);
        self.file_system.create_dir_all(&dest_path)?;

        println!("Copying files to: {}", dest_path.display());
        self.file_system.copy_dir_all(&source_path, &dest_path)?;

        println!("Successfully installed skill to: {}", dest_path.display());

        Ok(())
    }
}

pub fn extract_skill_name(path: &str) -> Result<String> {
    let path = path.trim_end_matches('/');
    let name = path
        .split('/')
        .last()
        .ok_or_else(|| anyhow!("Could not extract skill name from path"))?;
    Ok(name.to_string())
}
