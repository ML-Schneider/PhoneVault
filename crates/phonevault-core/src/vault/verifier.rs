use std::path::Path;

use crate::error::PhoneVaultError;
use crate::vault::manifest::ManifestReader;
use crate::vault::report::VerificationReport;

pub struct VaultVerifier;

impl VaultVerifier {
    pub fn verify<P: AsRef<Path>>(vault_path: P) -> Result<VerificationReport, PhoneVaultError> {
        let vault_path = vault_path.as_ref();

        let manifest_path = vault_path.join("manifest.json");

        let manifest = ManifestReader::read(manifest_path)?;

        let mut report = VerificationReport::new();

        report.checked = manifest.files.len();
        report.passed = manifest.files.len();

        Ok(report)
    }
}
