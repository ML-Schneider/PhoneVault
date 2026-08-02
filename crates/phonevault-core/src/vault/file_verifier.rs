use std::path::Path;

use crate::crypto::hasher::FileHasher;
use crate::error::PhoneVaultError;

pub struct VaultFileVerifier;

impl VaultFileVerifier {
    pub fn verify<P: AsRef<Path>>(file: P, expected_hash: &str) -> Result<bool, PhoneVaultError> {
        let actual_hash = FileHasher::sha256(file)?;

        Ok(actual_hash == expected_hash)
    }
}
