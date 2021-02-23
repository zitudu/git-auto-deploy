use serde::Deserialize;

pub struct Config {
    pub addr: String,
    pub webhook: Webhook,
}

pub struct Webhook {
    pub path: String,
}

pub enum RepositoryType {
    GITLAB,
    GITHUB,
}

pub enum RepositoryTarget {
    BRANCH(String),
    TAG(String),
}

pub struct RepositoryWebhook {
    pub enable: bool,
    pub typ: RepositoryType,
    pub path: String,
    pub rule: Option<String>,
}

pub struct RepositoryPoll {
    pub enable: bool,
    pub cron: String,
}

pub struct Repository {
    pub url: String,
    pub target: RepositoryTarget,
    pub webhook: Option<RepositoryWebhook>,
    pub poll: Option<repositoryPoll>,
}