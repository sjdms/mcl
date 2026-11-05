use crate::fsutil::write_atomic;

#[test]
fn test_atomic_write() {
    use std::fs;
    use std::io::Read;
    use tempfile::tempdir;

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_file.txt");
    let data = b"Hello, world!";

    // Write data atomically
    let write_result = tokio_test::block_on(write_atomic(&file_path, data));
    assert!(write_result.is_ok());

    // Read the file back and verify its contents
    let mut file = fs::File::open(&file_path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    assert_eq!(contents, data);

    // Clean up
    dir.close().unwrap();
}