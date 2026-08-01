#[derive(Debug, Clone)]
pub struct FileRecord {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub file_type: String,
    pub hash: Option<String>,
}
