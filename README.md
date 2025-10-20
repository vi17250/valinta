# Valinta 🟢

> *Valinta* a simple Rust crate 🦀 for multiple selection in the terminal

## Usage

```rust
use valinta::multi_select::selection;

let lines: Vec<i32> = vec![1,2,3,4];
let selected: Vec<i32> = selection(lines);
```
