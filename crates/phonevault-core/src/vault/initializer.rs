use std::path::Path;

use crate::error::PhoneVaultError;
use crate::vault::creator::VaultCreator;
use crate::vault::manifest::ManifestWriter;

pub struct VaultInitializer;

impl VaultInitializer {
    pub fn initialize<P: AsRef<Path>>(
        location: P,
    ) -> Result<(), PhoneVaultError> {

        VaultCreator::create(&location)?;

        let vault_path = location
            .as_ref()
            .join("PhoneVault");

        ManifestWriter::write(vault_path)?;

        Ok(())
    }
}