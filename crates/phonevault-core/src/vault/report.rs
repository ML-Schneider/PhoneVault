#[derive(Debug)]
pub struct VerificationReport {
    pub checked: usize,

    pub passed: usize,

    pub failed: usize,
}

impl VerificationReport {
    pub fn new() -> Self {
        Self {
            checked: 0,
            passed: 0,
            failed: 0,
        }
    }
}
impl Default for VerificationReport {
    fn default() -> Self {
        Self::new()
    }
}
