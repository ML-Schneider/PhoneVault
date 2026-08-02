use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::error::PhoneVaultError;

pub struct FileHasher;

impl FileHasher {
    pub fn sha256<P: AsRef<Path>>(path: P) -> Result<String, PhoneVaultError> {
        let file = File::open(path)?;

        let mut reader = BufReader::new(file);

        let mut hasher = Sha256::new();

        let mut buffer = [0u8; 8192];

        loop {
            let bytes_read = reader.read(&mut buffer)?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        let hash = hasher.finalize();

        Ok(hex::encode(hash))
    }
}
#[cfg(test)]
mod tests {
    use super::FileHasher;
    use std::fs;

    #[test]
    fn hashes_file() {
        let path = "test_hash.txt";

        fs::write(path, "hello world").unwrap();

        let hash = FileHasher::sha256(path).unwrap();

        assert_eq!(hash.len(), 64);

        fs::remove_file(path).unwrap();
    }
}
