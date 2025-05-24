# file-exists-macro

A Rust procedural macro that checks if a file exists at compile time and expands to a boolean (`true` or `false`) based on the file's existence.

## Example

```rust
use file_exists_macro::exists;

const CARGO_EXISTS: bool = exists!("Cargo.toml");

fn main() {
    if CARGO_EXISTS {
        println!("Cargo.toml file exists!");
    } else {
        println!("Cargo.toml file does not exist.");
    }
}
```