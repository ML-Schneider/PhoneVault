use std::path::Path;

use crate::error::PhoneVaultError;
use crate::vault::manifest::{Manifest, ManifestFile, ManifestReader, ManifestWriter};

pub struct ManifestService;

impl ManifestService {
    pub fn load_or_create<P: AsRef<Path>>(manifest_path: P) -> Result<Manifest, PhoneVaultError> {
        let path = manifest_path.as_ref();

        if path.exists() {
            ManifestReader::read(path)
        } else {
            Ok(Manifest::new())
        }
    }

    pub fn add_file(manifest: &mut Manifest, file: ManifestFile) {
        manifest.files.push(file);
    }

    pub fn save<P: AsRef<Path>>(manifest: &Manifest, path: P) -> Result<(), PhoneVaultError> {
        ManifestWriter::write(manifest, path)
    }
}
