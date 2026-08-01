use std::fs;
use std::path::Path;

use crate::error::PhoneVaultError;


pub struct FileCopier;


impl FileCopier {

    pub fn copy<P: AsRef<Path>>(
        source: P,
        destination: P,
    ) -> Result<(), PhoneVaultError> {

        fs::copy(
            source,
            destination,
        )?;

        Ok(())
    }
}