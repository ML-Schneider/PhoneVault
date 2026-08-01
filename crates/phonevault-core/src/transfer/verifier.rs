use std::path::Path;

use crate::inventory::fingerprint::Fingerprinter;


pub struct FileVerifier;


impl FileVerifier {

    pub fn verify<O: AsRef<Path>, C: AsRef<Path>>(
    original: O,
    copy: C,
) -> bool {

        let original_hash =
            Fingerprinter::hash_file(original);

        let copy_hash =
            Fingerprinter::hash_file(copy);


        match (
            original_hash,
            copy_hash
        ) {

            (Ok(first), Ok(second)) => {
                first == second
            }

            _ => false,
        }
    }
}