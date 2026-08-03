#[derive(Debug)]
pub struct VerificationReport {
    pub checked: usize,
    pub passed: usize,
    pub failed: usize,

    pub missing: Vec<String>,
    pub corrupted: Vec<String>,
}

impl VerificationReport {
    pub fn new() -> Self {
        Self {
            checked: 0,
            passed: 0,
            failed: 0,

            missing: Vec::new(),
            corrupted: Vec::new(),
        }
    }
}

impl Default for VerificationReport {
    fn default() -> Self {
        Self::new()
    }
}
