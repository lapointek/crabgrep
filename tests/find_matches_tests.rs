use assert_cmd::cargo::*;
use assert_fs::fixture::FileWriteStr;
use crabgrep::find_matches::contains_pattern;
use predicates::prelude::*;

#[test]
pub fn test_pattern() {
    assert!(contains_pattern("Hello World", "Hello"));
    assert!(!contains_pattern("Hello World", "bye"));
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("cgrep");

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = cargo_bin_cmd!("cgrep");
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Another testkk"));

    Ok(())
}
