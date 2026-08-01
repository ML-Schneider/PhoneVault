use std::fs;
use std::path::Path;

use crate::vault::manifest::{Manifest, ManifestWriter};


pub struct VaultInitializer;


impl VaultInitializer {

    pub fn initialize<P: AsRef<Path>>(
        vault_path: P,
    ) -> Result<(), std::io::Error> {

        let vault_path =
            vault_path.as_ref();


        fs::create_dir_all(
            vault_path
        )?;


        let manifest =
            Manifest::new();


        let manifest_path =
            vault_path.join("manifest.json");


        ManifestWriter::write(
            &manifest,
            manifest_path,
        )?;


        Ok(())
    }
}