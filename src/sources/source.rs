#![allow(unused)]
use super::{memfile::Memfile, profile::Profile};
use std::error::Error;
pub trait Source {
    fn get_base_url(&self) -> &str;
    fn set_base_url(&mut self, url: &str);
    fn get_latest_version_by_mod_id(
        &self,
        mod_id: &str,
        profile: &Profile,
    ) -> Result<String, Box<dyn Error>>;

    fn get_file_by_version(&self, version_id: &str) -> Result<Memfile, Box<dyn Error>>;
}
