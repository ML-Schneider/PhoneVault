use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::PhoneVaultError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub vault_version: String,
    pub files: Vec<ManifestFile>,
}

impl Manifest {
    pub fn new() -> Self {
        Self {
            vault_version: "0.2".to_string(),
            files: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManifestFile {
    /// Path relative to the Content/ directory.
    pub path: String,

    pub size: u64,

    pub hash: String,
}

pub struct ManifestWriter;

impl ManifestWriter {
    pub fn write<P: AsRef<Path>>(manifest: &Manifest, path: P) -> Result<(), PhoneVaultError> {
        let json = serde_json::to_string_pretty(manifest)?;

        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }
}

pub struct ManifestReader;

impl ManifestReader {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Manifest, PhoneVaultError> {
        let file = File::open(path)?;

        let manifest = serde_json::from_reader(file)?;

        Ok(manifest)
    }
}
impl Default for Manifest {
    fn default() -> Self {
        Self::new()
    }
}
