//! From: https://github.com/WaffleHacks/wafflemaker/blob/cb9bef665c49fe04112cac0d7e9a7e1b568f014f/src/webhooks/models/github.rs

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "lowercase")]
pub struct GithubHook {
    #[serde(rename = "ref")]
    pub reference: String,
    pub repository: Repository,
    pub pusher: Pusher,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged, rename_all = "lowercase")]
pub struct Repository {
    #[serde(rename = "full_name")]
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged, rename_all = "lowercase")]
pub struct Pusher {
    pub date: String,
    pub email: Option<String>,
    pub name: String,
    pub username: String,
}