use std::fs::read_to_string;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manifest {
    pub entry: String,
    pub metadata: Option<Metadata>,
    pub settings: Settings
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

impl Manifest {
    pub fn from_string(string: String) -> Option<Manifest> {
        match toml::from_str(&string) {
            Ok(manifest) => Some(manifest),
            Err(e) => {None}
        }
    }

    pub fn from_file(filename: PathBuf) -> Option<Manifest> {
        let text = read_to_string(filename).unwrap();
        Self::from_string(text)
    }

    pub fn to_string(&self) -> String {
        toml::to_string_pretty(self).unwrap()
    }
}