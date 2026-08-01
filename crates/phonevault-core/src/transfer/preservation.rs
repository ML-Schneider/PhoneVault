use std::path::PathBuf;

use crate::inventory::scanner::Scanner;
use crate::inventory::service::InventoryService;

use crate::transfer::copier::FileCopier;
use crate::transfer::verifier::FileVerifier;

use crate::vault::manifest::ManifestFile;
use crate::vault::service::ManifestService;

use super::report::TransferReport;


pub struct PreservationJob {

    pub source: PathBuf,

    pub destination: PathBuf,
}


impl PreservationJob {

    pub fn new(
        source: PathBuf,
        destination: PathBuf,
    ) -> Self {

        Self {
            source,
            destination,
        }
    }


    pub fn execute(
        &self,
    ) -> TransferReport {

        let records =
            Scanner::scan(&self.source);


        let records =
            InventoryService::fingerprint_records(records);


        let mut report =
            TransferReport::new();


            let manifest_path =
    self.destination.join("manifest.json");


let mut manifest =
    ManifestService::load_or_create(
        &manifest_path
    )
    .unwrap();


        report.files_scanned =
            records.len();


        for record in records {

            let destination =
                self.destination.join(&record.name);


            match FileCopier::copy(
                &record.path,
                &destination,
            ) {

                Ok(_) => {

                    report.files_copied += 1;


                    if FileVerifier::verify(
                        &record.path,
                        &destination,
                    ) {

                        report.files_verified += 1;
                        ManifestService::add_file(
    &mut manifest,
   ManifestFile {
    name: record.name.clone(),
    size: record.size,
    hash: record.hash.clone().unwrap_or_else(|| {
        "UNKNOWN".to_string()
    }),
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

ManifestService::save(
    &manifest,
    &manifest_path,
)
.unwrap();


        report
    }
}