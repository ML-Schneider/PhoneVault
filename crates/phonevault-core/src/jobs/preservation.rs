use std::path::PathBuf;

use crate::error::PhoneVaultError;

use crate::inventory::scanner::Scanner;

use crate::transfer::copier::FileCopier;
use crate::transfer::verifier::FileVerifier;

use crate::vault::manifest::ManifestFile;
use crate::vault::service::ManifestService;

use crate::transfer::report::TransferReport;

pub struct PreservationJob {
    pub source: PathBuf,
    pub destination: PathBuf,
}

impl PreservationJob {
    pub fn new(source: PathBuf, destination: PathBuf) -> Self {
        Self {
            source,
            destination,
        }
    }

    pub fn execute(&self) -> Result<TransferReport, PhoneVaultError> {
        let records = Scanner::scan(&self.source)?;

        let mut report = TransferReport::new();

        let content_root = self.destination.join("Content");

std::fs::create_dir_all(&content_root)?;

        let manifest_path = self.destination.join("manifest.json");

        let mut manifest = ManifestService::load_or_create(&manifest_path)?;

        report.files_scanned = records.len();

        for record in records {
            let source_path = std::path::Path::new(&record.path);

let relative_path = source_path
    .strip_prefix(&self.source)
    .unwrap();

let destination = content_root.join(relative_path);

if let Some(parent) = destination.parent() {
    std::fs::create_dir_all(parent)?;
}

            match FileCopier::copy(&record.path, &destination) {
                Ok(_) => {
                    report.files_copied += 1;

                    if FileVerifier::verify(&record.path, &destination) {
                        report.files_verified += 1;

                        ManifestService::add_file(
                            &mut manifest,
                            ManifestFile {
                                name: record.name.clone(),
                                size: record.size,
                                hash: record.hash.clone().unwrap_or_else(|| "UNKNOWN".to_string()),
                            },
                        );
                    } else {
                        report.failures += 1;
                    }
                }

                Err(_) => {
                    report.failures += 1;
                }
            }
        }

        ManifestService::save(&manifest, &manifest_path)?;

        Ok(report)
    }
}
