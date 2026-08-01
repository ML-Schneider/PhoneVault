use std::fs;
use std::path::Path;

use crate::error::PhoneVaultError;

pub struct FileCopier;

impl FileCopier {
    pub fn copy<S: AsRef<Path>, D: AsRef<Path>>(
        source: S,
        destination: D,
    ) -> Result<(), PhoneVaultError> {
        let destination = destination.as_ref();

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(source, destination)?;

        Ok(())
    }
}
