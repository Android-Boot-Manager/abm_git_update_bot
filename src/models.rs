//! From: https://github.com/WaffleHacks/wafflemaker/blob/cb9bef665c49fe04112cac0d7e9a7e1b568f014f/src/webhooks/models/github.rs

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GithubHook {
    #[serde(rename = "ref")]
    pub reference: String,
    pub repository: Repository,
    pub pusher: Pusher,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Repository {
    #[serde(rename = "full_name")]
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Pusher {
    pub date: String,
    pub email: Option<String>,
    pub name: String,
    pub username: String,
}
