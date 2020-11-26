use doc_comment::doctest;

// This should make `cargo test` fail, but it doesn't actually run:
doctest!("../README.md");
