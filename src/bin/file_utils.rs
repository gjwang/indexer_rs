use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Error, Write};

use std::thread::sleep;
use std::time::Duration;
pub fn create_file_if_not_exists(path: &str) -> std::io::Result<File> {
    OpenOptions::new().write(true).create(true).open(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::time;

    const TEST_FILE: &str = "test_file.txt";

    fn cleanup() {
        //Maybe rust is running too fast,
        // so there a lot of chances the test will failed if not delay a little time
        sleep(Duration::from_millis(10));
        if Path::new(TEST_FILE).exists() {
            fs::remove_file(TEST_FILE).unwrap();
            sleep(Duration::from_millis(10));

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
            // Ensure all data is flushed to the disk
            file.flush().unwrap();
        }

        let before = fs::read(TEST_FILE).unwrap();
        create_file_if_not_exists(TEST_FILE).unwrap();
        let after = fs::read(TEST_FILE).unwrap();

        assert_eq!(before, after); // Ensure the file content hasn't changed

        cleanup();
    }

    #[test]
    #[should_panic]
    fn fails_with_invalid_file_name() {
        create_file_if_not_exists("").unwrap();
    }
}

fn main() {

}