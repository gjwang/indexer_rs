// src/utils/file_utils.rs
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Error, Write};

pub fn create_file_if_not_exists(path: &str) -> std::io::Result<File> {
    OpenOptions::new().write(true).create(true).open(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const TEST_FILE: &str = "test_file.txt";

    fn cleanup() {
        if Path::new(TEST_FILE).exists() {
            fs::remove_file(TEST_FILE).unwrap();
        }
    }

    #[test]
    fn creates_file_if_not_exists() {
        cleanup(); // Ensure the file doesn't exist at the start
        assert!(!Path::new(TEST_FILE).exists());

        create_file_if_not_exists(TEST_FILE).unwrap();
        assert!(Path::new(TEST_FILE).exists());

        cleanup();
    }

    #[test]
    fn does_not_overwrite_existing_file() {
        cleanup();

        // Create and write some data to the file
        {
            let mut file = fs::File::create(TEST_FILE).unwrap();
            file.write_all(b"Some data").unwrap();
        }

        let before = fs::read(TEST_FILE).unwrap();
        create_file_if_not_exists(TEST_FILE).unwrap();
        let after = fs::read(TEST_FILE).unwrap();

        println!("before {:?}", before);
        println!("after  {:?}", after);

        assert_eq!(before, after); // Ensure the file content hasn't changed

        cleanup();
    }

    #[test]
    #[should_panic]
    fn fails_with_invalid_file_name() {
        create_file_if_not_exists("").unwrap();
    }
}
