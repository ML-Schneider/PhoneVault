use std::fmt;

#[derive(Debug)]
pub enum PhoneVaultError {
    Io(std::io::Error),
}

impl From<std::io::Error> for PhoneVaultError {
    fn from(error: std::io::Error) -> Self {
        PhoneVaultError::Io(error)
    }
}

impl fmt::Display for PhoneVaultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhoneVaultError::Io(error) => {
                write!(f, "File system error: {}", error)
            }
        }
    }
}