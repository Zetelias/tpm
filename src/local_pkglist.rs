use serde::{Deserialize, Serialize};

/// The local package list is a list of packages that are installed.
/// It is stored in a JSON file in the user's home directory.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalPkgList {
    pub packages: Vec<String>
}

impl LocalPkgList {
    pub fn new(packages: Vec<String>) -> Self {
        Self {
            packages,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    pub fn insert(&mut self, name: String) {
        self.packages.push(name);
    }

    pub fn remove(&mut self, name: &str) {
        self.packages.retain(|x| x != name);
    }

    pub fn contains(&self, name: &str) -> bool {
        self.packages.contains(&name.to_string())
    }

    pub fn load() -> Option<Self> {
        let path = dirs::home_dir().unwrap().join(".tpm_local_pkglist.json");
        if path.exists() {
            let file = std::fs::File::open(path).unwrap();
            let reader = std::io::BufReader::new(file);
            let local_pkglist: LocalPkgList = serde_json::from_reader(reader).unwrap();
            Some(local_pkglist)
        } else {
            None
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = dirs::home_dir().unwrap_or(
            std::path::PathBuf::from(".")
        ).join(".tpm_local_pkglist.json");
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }
}