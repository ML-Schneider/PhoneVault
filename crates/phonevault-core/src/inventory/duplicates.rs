use super::record::FileRecord;


pub struct DuplicateDetector;


impl DuplicateDetector {

    pub fn are_duplicates(
        first: &FileRecord,
        second: &FileRecord,
    ) -> bool {

        match (&first.hash, &second.hash) {

            (Some(first_hash), Some(second_hash)) => {
                first_hash == second_hash
            }

            _ => false,
        }
    }
}