use std::path::Path;

use walkdir::WalkDir;

use super::record::FileRecord;

pub struct Scanner;

impl Scanner {
    pub fn scan<P: AsRef<Path>>(
        location: P,
    ) -> Vec<FileRecord> {

        let mut records = Vec::new();

        for entry in WalkDir::new(location)
            .into_iter()
            .filter_map(Result::ok)
        {
            if entry.file_type().is_file() {

                let metadata = match entry.metadata() {
                    Ok(data) => data,
                    Err(_) => continue,
                };

                let path = entry.path();

                let record = FileRecord {
                    name: path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),

                    path: path
                        .to_string_lossy()
                        .to_string(),

                    size: metadata.len(),

                    file_type: detect_file_type(path),
                };

                records.push(record);
            }
        }

        records
    }
}


fn detect_file_type(path: &Path) -> String {

    match path.extension()
        .and_then(|x| x.to_str())
    {
        Some("jpg") |
        Some("jpeg") |
        Some("png") |
        Some("heic") => "image".to_string(),

        Some("mp4") |
        Some("mov") => "video".to_string(),

        Some("mp3") |
        Some("wav") |
        Some("m4a") => "audio".to_string(),

        Some("pdf") |
        Some("doc") |
        Some("docx") => "document".to_string(),

        _ => "other".to_string(),
    }
}