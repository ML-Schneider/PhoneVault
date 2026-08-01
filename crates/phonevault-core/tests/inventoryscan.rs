use std::fs;
use std::fs::File;

use phonevault_core::inventory::scanner::Scanner;

#[test]
fn scanner_finds_files() {
    let test_dir = std::env::temp_dir().join("phonevault_inventory_test");

    let _ = fs::remove_dir_all(&test_dir);

    fs::create_dir_all(&test_dir).unwrap();

    File::create(test_dir.join("photo.jpg")).unwrap();

    File::create(test_dir.join("song.mp3")).unwrap();

    File::create(test_dir.join("document.pdf")).unwrap();

    let records = Scanner::scan(&test_dir);

    assert_eq!(records.len(), 3);

    fs::remove_dir_all(&test_dir).unwrap();
}
