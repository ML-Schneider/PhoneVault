use std::path::PathBuf;

use crate::inventory::scanner::Scanner;
use crate::inventory::service::InventoryService;

use crate::transfer::copier::FileCopier;
use crate::transfer::verifier::FileVerifier;

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

                    } else {

                        report.failures += 1;

                    }
                }


                Err(_) => {

                    report.failures += 1;

                }
            }
        }


        report
    }
}