#![allow(unused)]
use crate::sources::memfile::Memfile;
use crate::sources::modrinth_types::Project;
use crate::sources::modrinth_types::Version;
use crate::sources::profile::Profile;
use crate::sources::source::Source;
use reqwest;
use std::error::Error;

pub struct ModrinthSource {
    base: String,
}

impl ModrinthSource {
    pub fn new() -> Self {
        Self {
            base: "https://api.modrinth.com/v2/".to_string(), // change to api.modrinth.com when done
        }
    }

    pub fn get_project_by_id(&self, id: &str) -> Result<Project, Box<dyn Error>> {
        let url = format!("{}{}{}", self.base, "project/", id);
        let resp = reqwest::blocking::get(&url)?.text()?;
        let project: Project = serde_json::from_str(&resp)?;
        Ok(project)
    }

    pub fn get_versions(
        &self,
        profile: &Profile,
        project: &Project,
    ) -> Result<Vec<Version>, Box<dyn Error>> {
        let params = format!(
            "?game_versions=[\"{}\"]&loaders=[\"{}\"]",
            profile.game_version, profile.loader
        );
        let url = format!(
            "{}{}{}{}{}",
            self.base, "project/", project.id, "/version", params
        );
        let resp = reqwest::blocking::get(&url)?.text()?;
        let versions: Vec<Version> = serde_json::from_str(&resp)?;
        Ok(versions)
    }

    pub fn get_latest_version(&self, versions: Vec<Version>) -> Version {
        let mut latest_version = versions[0].clone();
        for version in versions {
            if version.featured {
                latest_version = version;
                break;
            }
        }
        latest_version
    }
}

impl Source for ModrinthSource {
    fn get_base_url(&self) -> &str {
        &self.base
    }

    fn set_base_url(&mut self, url: &str) {
        self.base = url.to_string();
    }

    fn get_latest_version_by_mod_id(
        &self,
        mod_id: &str,
        profile: &Profile,
    ) -> Result<String, Box<dyn Error>> {
        // Get project by id
        // List numbers
        // Get latest version number
        // Call file from link
        let proj = self.get_project_by_id(mod_id)?;

        let versions = self.get_versions(profile, &proj)?;
        let latest_version = self.get_latest_version(versions);
        Ok(latest_version.id)

        /*
         * For next time: return the version id
         * make a new function 'download file from version id'
         ** after: make functions to find and install dependencies
         ** handle staging
         ** cli
         ** gui
         */
    }

    fn get_file_by_version(&self, version_id: &str) -> Result<Memfile, Box<dyn Error>> {
        let url = format!("{}{}{}", self.base, "version/", version_id);
        let version_response = reqwest::blocking::get(&url)?.text()?;
        let version: Version = serde_json::from_str(&version_response)?;

        let version_file = &version.files[0];
        let file_content = reqwest::blocking::get(&version_file.url)?.text()?;
        println!("Downloading mod {}", version_file.filename.clone());
        let memory_file = Memfile::new(version_file.filename.clone(), file_content);
        Ok(memory_file)
    }
}
