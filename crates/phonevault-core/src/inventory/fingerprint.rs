use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

use sha2::{Digest, Sha256};


pub struct Fingerprinter;


impl Fingerprinter {

    pub fn hash_file<P: AsRef<Path>>(
        path: P,
    ) -> Result<String> {

        let mut file = File::open(path)?;

        let mut hasher = Sha256::new();

        let mut buffer = [0u8; 8192];


        loop {

            let bytes_read =
                file.read(&mut buffer)?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(
                &buffer[..bytes_read]
            );
        }


        let result = hasher.finalize();


        Ok(
            format!("{:x}", result)
        )
    }
}