# Valinta 🟢

> *Valinta* a simple Rust crate 🦀 for multiple selection in the terminal

## Usage

```rust
use valinta;
fn main() {
    let lines: Vec<i32> = vec![1, 2, 3, 4];
    let _: Vec<i32> = valinta::multi_select(&lines).unwrap();
}

```
