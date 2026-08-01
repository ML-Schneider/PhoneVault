use std::fs;

use phonevault_core::vault::creator::VaultCreator;

#[test]
fn creates_digital_vault_structure() {
    let test_location = std::env::temp_dir()
        .join("phonevault_test");

    let _ = fs::remove_dir_all(&test_location);

    VaultCreator::create(&test_location)
        .expect("Vault creation failed");

    assert!(
        test_location
            .join("PhoneVault/Digital Vault/Memories/Photos")
            .exists()
    );

    assert!(
        test_location
            .join("PhoneVault/Digital Vault/Files/Music")
            .exists()
    );

    fs::remove_dir_all(&test_location)
        .unwrap();
}