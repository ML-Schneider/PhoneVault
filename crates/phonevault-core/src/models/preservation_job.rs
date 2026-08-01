use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub enum JobStatus {
    Created,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct PreservationJob {
    pub id: String,
    pub name: String,
    pub source: String,
    pub destination: String,
    pub status: JobStatus,
    pub created_at: u64,
}

impl PreservationJob {
    pub fn new(
        name: String,
        source: String,
        destination: String,
    ) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id: format!("job-{}", created_at),
            name,
            source,
            destination,
            status: JobStatus::Created,
            created_at,
        }
    }
}