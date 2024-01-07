use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::global_pkglist::GlobalPkgList;
use crate::local_pkglist::LocalPkgList;
use crate::platform::PlatformSpecificInfo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub platforms: HashMap<String, PlatformSpecificInfo>
}

impl Package {
    pub fn new(
        name: String,
        version: String,
        description: String,
        platforms: HashMap<String, PlatformSpecificInfo>
    ) -> Self {
        Self {
            name,
            version,
            description,
            platforms,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            description: String::new(),
            platforms: HashMap::new(),
        }
    }
}