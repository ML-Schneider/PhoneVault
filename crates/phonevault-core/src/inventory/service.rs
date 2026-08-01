use super::fingerprint::Fingerprinter;
use super::record::FileRecord;


pub struct InventoryService;


impl InventoryService {

    pub fn fingerprint_records(
        mut records: Vec<FileRecord>,
    ) -> Vec<FileRecord> {

        for record in records.iter_mut() {

            if let Ok(hash) =
                Fingerprinter::hash_file(&record.path)
            {
                record.hash = Some(hash);
            }

        }

        records
    }
}