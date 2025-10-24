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

# With `result<T,E>`
```rust
use valinta::{ValintaError, multi_select};
fn main() -> Result<(), ValintaError> {
    let lines: Vec<i32> = vec![1, 2, 3, 4];
    let _: Vec<i32> = multi_select(&lines)?;
    Ok(())
}
```
