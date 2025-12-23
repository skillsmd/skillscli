use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct GitHubRepo {
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubContent {
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketEntry {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct SkillMatch {
    pub name: String,
    pub url: String,
    pub market_name: String,
}
