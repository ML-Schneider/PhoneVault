use std::fs;
use std::fs::File;

use phonevault_core::jobs::preservation::PreservationJob;

#[test]
fn preservation_job_copies_and_verifies_files() {
    let source = std::env::temp_dir().join("phonevault_source_test");

    let destination = std::env::temp_dir().join("phonevault_destination_test");

    let _ = fs::remove_dir_all(&source);

    let _ = fs::remove_dir_all(&destination);

    fs::create_dir_all(&source).unwrap();

    fs::create_dir_all(&destination).unwrap();

    File::create(source.join("photo.jpg")).unwrap();

    File::create(source.join("song.mp3")).unwrap();

    let job = PreservationJob::new(source.clone(), destination.clone());

    let report = job
    .execute()
    .expect("preservation job should succeed");

    assert_eq!(report.files_scanned, 2);

    assert_eq!(report.files_copied, 2);

    assert_eq!(report.files_verified, 2);

    assert_eq!(report.failures, 0);

    fs::remove_dir_all(&source).unwrap();

    fs::remove_dir_all(&destination).unwrap();
}
