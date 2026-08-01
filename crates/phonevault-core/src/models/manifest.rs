use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub application: String,
    pub version: String,
    pub creator: String,
    pub vault_type: String,
    pub created_at: u64,
    pub vault_id: String,
}

impl Manifest {
    pub fn new() -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            application: "PhoneVault".to_string(),
            version: "0.1.0".to_string(),
            creator: "Malachi Schneider".to_string(),
            vault_type: "Digital Vault".to_string(),
            created_at,
            vault_id: format!("vault-{}", created_at),
        }
    }
}
impl Default for Manifest {
    fn default() -> Self {
        Self::new()
    }
}