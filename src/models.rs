//! From: https://github.com/WaffleHacks/wafflemaker/blob/cb9bef665c49fe04112cac0d7e9a7e1b568f014f/src/webhooks/models/github.rs

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct GithubHook {
    #[serde(rename = "ref")]
    pub(crate) reference: String,
    pub(crate) repository: Repository,
    pub(crate) pusher: Pusher,
    pub(crate) head_commit: Commit,
    pub(crate) commits: Vec<Commit>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Repository {
    #[serde(rename = "full_name")]
    pub(crate) name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Commit {
    pub(crate) message: Option<String>,
    pub(crate) author: Pusher,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Pusher {
    pub(crate) email: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) username: Option<String>,
}
