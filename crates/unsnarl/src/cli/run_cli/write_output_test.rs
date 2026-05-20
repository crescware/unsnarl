use super::*;
use std::fs;

#[test]
fn writes_text_to_stdout_when_output_path_is_none() {
    let mut buf = Vec::new();
    write_output(None, "hello\n", &mut buf).expect("write_output should succeed");
    assert_eq!(buf, b"hello\n");
}

#[test]
fn writes_empty_text_to_stdout_when_output_path_is_none() {
    let mut buf = Vec::new();
    write_output(None, "", &mut buf).expect("write_output should succeed");
    assert!(buf.is_empty());
}

#[test]
fn writes_text_to_the_given_path() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("out.txt");
    let mut buf = Vec::new();
    write_output(Some(&path), "graph contents\n", &mut buf).expect("write_output should succeed");
    assert_eq!(
        fs::read_to_string(&path).expect("file readable"),
        "graph contents\n"
    );
    assert!(buf.is_empty(), "stdout should remain untouched");
}

#[test]
fn creates_missing_parent_directories_when_writing_to_a_nested_path() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("nested/deeper/out.mmd");
    let mut buf = Vec::new();
    write_output(Some(&path), "x", &mut buf).expect("write_output should succeed");
    assert_eq!(fs::read_to_string(&path).expect("file readable"), "x");
}

#[test]
fn overwrites_an_existing_file_at_the_target_path() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("out.txt");
    fs::write(&path, "stale").expect("seed file");
    let mut buf = Vec::new();
    write_output(Some(&path), "fresh", &mut buf).expect("write_output should succeed");
    assert_eq!(fs::read_to_string(&path).expect("file readable"), "fresh");
}
