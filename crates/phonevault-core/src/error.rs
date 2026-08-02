use std::fmt;
use std::io;

#[derive(Debug)]
pub enum PhoneVaultError {
    Io(io::Error),
    Json(serde_json::Error),
    WalkDir(walkdir::Error),

    InvalidManifest(String),
    VerificationFailed(String),
}

impl fmt::Display for PhoneVaultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhoneVaultError::Io(err) => {
                write!(f, "I/O error: {}", err)
            }

            PhoneVaultError::Json(err) => {
                write!(f, "JSON error: {}", err)
            }

            PhoneVaultError::WalkDir(err) => {
                write!(f, "Directory traversal error: {}", err)
            }

            PhoneVaultError::InvalidManifest(msg) => {
                write!(f, "Invalid manifest: {}", msg)
            }

            PhoneVaultError::VerificationFailed(msg) => {
                write!(f, "Verification failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for PhoneVaultError {}

impl From<io::Error> for PhoneVaultError {
    fn from(error: io::Error) -> Self {
        PhoneVaultError::Io(error)
    }
}

impl From<serde_json::Error> for PhoneVaultError {
    fn from(error: serde_json::Error) -> Self {
        PhoneVaultError::Json(error)
    }
}

impl From<walkdir::Error> for PhoneVaultError {
    fn from(error: walkdir::Error) -> Self {
        PhoneVaultError::WalkDir(error)
    }
}
