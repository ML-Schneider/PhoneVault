use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::error::PhoneVaultError;
use crate::models::manifest::Manifest;

pub struct ManifestWriter;

impl ManifestWriter {
    pub fn write<P: AsRef<Path>>(
        location: P,
    ) -> Result<(), PhoneVaultError> {

        let manifest = Manifest::new();

        let json = serde_json::to_string_pretty(&manifest)
            .unwrap();

        let path = location
            .as_ref()
            .join("manifest.json");

        let mut file = File::create(path)?;

        file.write_all(json.as_bytes())?;

        Ok(())
    }
}