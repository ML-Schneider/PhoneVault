#[derive(Debug, Default)]
pub struct TransferReport {

    pub files_scanned: usize,

    pub files_copied: usize,

    pub files_verified: usize,

    pub duplicates_found: usize,

    pub failures: usize,
}


impl TransferReport {

    pub fn new() -> Self {
        Self::default()
    }


    pub fn success_rate(&self) -> f64 {

        if self.files_copied == 0 {
            return 0.0;
        }

        self.files_verified as f64
            / self.files_copied as f64
    }
}