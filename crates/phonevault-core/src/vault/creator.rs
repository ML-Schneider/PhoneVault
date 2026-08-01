use std::fs;
use std::path::Path;

use crate::error::PhoneVaultError;

pub struct VaultCreator;

impl VaultCreator {
    pub fn create<P: AsRef<Path>>(location: P) -> Result<(), PhoneVaultError> {
        let root = location.as_ref().join("PhoneVault");

        let folders = [
            "Digital Vault/Memories/Photos",
            "Digital Vault/Memories/Videos",
            "Digital Vault/Memories/Live Photos",
            "Digital Vault/Memories/Voice Memos",
            "Digital Vault/Files/Documents",
            "Digital Vault/Files/Downloads",
            "Digital Vault/Files/Music",
            "Digital Vault/Files/Notes",
            "Digital Vault/Reports",
        ];

        for folder in folders {
            fs::create_dir_all(root.join(folder))?;
        }

        Ok(())
    }
}