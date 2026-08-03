use std::path::Path;

use crate::crypto::hasher::FileHasher;
use crate::error::PhoneVaultError;
use crate::vault::manifest::ManifestReader;
use crate::vault::report::VerificationReport;

pub struct VaultVerifier;

impl VaultVerifier {
    pub fn verify<P: AsRef<Path>>(
        vault_path: P,
    ) -> Result<VerificationReport, PhoneVaultError> {
        let vault_path = vault_path.as_ref();

        let manifest_path = vault_path.join("manifest.json");

        let manifest = ManifestReader::read(manifest_path)?;

        let mut report = VerificationReport::new();

        for file in manifest.files {
            report.checked += 1;

            let archived_file =
    vault_path
        .join("Content")
        .join(&file.path);

            if !archived_file.exists() {
                report.failed += 1;
                report.missing.push(file.name);
                continue;
            }

            let actual_hash = FileHasher::sha256(&archived_file)?;

            if actual_hash == file.hash {
                report.passed += 1;
            } else {
                report.failed += 1;
                report.corrupted.push(file.name);
            }
        }

        Ok(report)
    }
}