use std::fs::read_to_string;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manifest {
    pub metadata: Option<Metadata>,
    pub settings: Settings,
    pub resources: Resources
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub ram_size: u64,
    pub stack_size: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Resources {
    pub entry: String,
    pub palette: String,
}

impl Manifest {
    pub fn from_string(string: String) -> Option<Manifest> {
        match toml::from_str(&string) {
            Ok(manifest) => Some(manifest),
            Err(_) => {None}
        }
    }

    pub fn from_file(filename: &PathBuf) -> Option<Manifest> {
        let text = read_to_string(filename).unwrap();
        Self::from_string(text)
    }

    pub fn to_string(&self) -> String {
        toml::to_string_pretty(self).unwrap()
    }
}