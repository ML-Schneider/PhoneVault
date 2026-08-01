use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;


#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {

    pub vault_version: String,

    pub files: Vec<ManifestFile>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestFile {

    pub name: String,

    pub size: u64,

    pub hash: String,
}


impl Manifest {

    pub fn new() -> Self {

        Self {
            vault_version: "0.1".to_string(),
            files: Vec::new(),
        }
    }
}


pub struct ManifestWriter;


impl ManifestWriter {

    pub fn write<P: AsRef<Path>>(
        manifest: &Manifest,
        path: P,
    ) -> Result<(), std::io::Error> {

        let json =
            serde_json::to_string_pretty(manifest)
                .unwrap();


        let mut file =
            File::create(path)?;


        file.write_all(
            json.as_bytes()
        )?;


        Ok(())
    }
}


pub struct ManifestReader;


impl ManifestReader {

    pub fn read<P: AsRef<Path>>(
        path: P,
    ) -> Result<Manifest, std::io::Error> {

        let file =
            File::open(path)?;


        let manifest =
            serde_json::from_reader(file)
                .unwrap();


        Ok(manifest)
    }
}