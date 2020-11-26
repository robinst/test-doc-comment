Test for doc-comment crate, see
https://github.com/GuillaumeGomez/doc-comment/issues/25

This should make `cargo test` fail:

```rust
assert_eq!("foo", "bar");
```
