use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::package::Package;

/// The global package list is a list of packages that are available to install.
/// It is stored in a JSON file in the git repository.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalPkgList {
    pub packages: HashMap<String, Package>
}

impl GlobalPkgList {
    pub fn new(packages: HashMap<String, Package>) -> Self {
        Self {
            packages,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }

    pub fn insert(&mut self, package: Package) {
        self.packages.insert(package.name.clone(), package);
    }

    pub fn remove(&mut self, name: &str) {
        self.packages.remove(name);
    }

    pub fn get(&self, name: &str) -> Option<&Package> {
        self.packages.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Package> {
        self.packages.get_mut(name)
    }

    pub fn load_local() -> Option<Self> {
        let path = dirs::home_dir().unwrap().join(".tpm_local_global_pkglist.json");
        if path.exists() {
            let file = std::fs::File::open(path).unwrap();
            let reader = std::io::BufReader::new(file);
            let global_pkglist: GlobalPkgList = serde_json::from_reader(reader).unwrap();
            Some(global_pkglist)
        } else {
            None
        }
    }

    pub fn save_local(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = dirs::home_dir().unwrap_or(
            std::path::PathBuf::from(".")
        ).join(".tpm_local_global_pkglist.json");
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    pub fn load_from_url(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let response = reqwest::blocking::get(url)?;
        let global_pkglist: GlobalPkgList = response.json()?;
        Ok(global_pkglist)
    }
}