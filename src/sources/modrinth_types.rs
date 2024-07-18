#![allow(unused)]
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub categories: Vec<String>,
    pub client_side: String,
    pub server_side: String,
    pub body: Option<String>,
    pub status: String,
    pub requested_status: Option<String>,
    pub additional_categories: Option<Vec<String>>,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
    pub wiki_url: Option<String>,
    pub discord_url: Option<String>,
    pub donation_urls: Option<Vec<DonationUrl>>,
    pub project_type: String,
    pub downloads: u64,
    pub icon_url: Option<String>,
    pub color: Option<u32>,
    pub thread_id: Option<String>,
    pub monetization_status: Option<String>,
    pub id: String,
    pub team: String,
    pub body_url: Option<String>,
    pub moderator_message: Option<String>,
    pub published: Option<String>,
    pub updated: Option<String>,
    pub approved: Option<String>,
    pub queued: Option<String>,
    pub followers: u64,
    pub license: Option<License>,
    pub versions: Option<Vec<String>>,
    pub game_versions: Option<Vec<String>>,
    pub loaders: Option<Vec<String>>,
    pub gallery: Option<Vec<Gallery>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DonationUrl {
    pub id: String,
    pub platform: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct License {
    pub id: String,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Gallery {
    pub url: String,
    pub featured: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created: Option<String>,
    pub ordering: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    pub id: String,
    pub project_id: String,
    pub author_id: String,
    pub featured: bool,
    pub name: Option<String>,
    pub version_number: String,
    pub changelog: Option<String>,
    pub changelog_url: Option<String>,
    pub date_published: String,
    pub downloads: u64,
    pub version_type: String,
    pub status: String,
    pub requested_status: Option<String>,
    pub loaders: Vec<String>,
    pub game_versions: Vec<String>,
    pub dependencies: Vec<Dependency>,
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dependency {
    pub version_id: Option<String>,
    pub project_id: Option<String>,
    pub dependency_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hashes {
    pub sha1: String,
    pub sha512: String,
}
