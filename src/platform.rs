use serde::{Deserialize, Serialize};
use crate::build_script::BuildScript;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlatformSpecificInfo {
    pub download_urls: Vec<String>,
    pub dependencies: Vec<String>,
    pub build_script: BuildScript,
}

impl PlatformSpecificInfo {
    pub fn new(
        download_urls: Vec<String>,
        dependencies: Vec<String>,
        build_script: BuildScript,
    ) -> Self {
        Self {
            download_urls,
            dependencies,
            build_script,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            download_urls: Vec::new(),
            dependencies: Vec::new(),
            build_script: BuildScript::None,
        }
    }
}