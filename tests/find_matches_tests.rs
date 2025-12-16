use crabgrep::find_matches::contains_pattern;

#[test]
pub fn test_pattern() {
    assert!(contains_pattern("Hello World", "Hello"));
    assert!(!contains_pattern("Hello World", "bye"));
}
