# [x] Valinta

> _Valinta_ a zero config Rust crate 🦀 for multiple selection in the terminal

# Usage

## First: add this crate to your project

```bash
cargo add valinta
```

## Next:

```rust
use valinta::select;

fn main() -> std::io::Result<()> {

    let animals = vec![
        "🦍 gorilla",
        "🪼 jellyfish",
        "🦁 lion",
        "🐝 honeybee",
        "🐗 boar",
        "🦇 bat",
        "🐌 snail",
        "🐨 koala",
        "🦉 owl",
        "🐢 turtle",
        "🐬 dolphin",
    ];

    let selected_animals = select(&animals).expect("Error message");

    Ok(())
}
```

## Or by using _ValintaError_

```rust
use valinta::{select, ValintaError};
fn main() -> Result<(), ValintaError> {

    let animals = vec![
        "🦍 gorilla",
        "🪼 jellyfish",
        "🦁 lion",
        "🐝 honeybee",
        "🐗 boar",
        "🦇 bat",
        "🐌 snail",
        "🐨 koala",
        "🦉 owl",
        "🐢 turtle",
        "🐬 dolphin",
    ];

    let selected_animals = select(&animals)?;

    Ok(())
}
```

# User interactions

| _key pressed_  | Action            |
| -------------- | ----------------- |
| _↓_            | Next item         |
| _↑_            | Previous item     |
| _a_ **or** _A_ | Select all        |
| _n_ **or** _N_ | Deselect all      |
| _i_ **or** _I_ | Invert selection  |
| _space_        | Toggle current    |
| _enter_        | Confirm selection |
| _esc_          | Break             |

# Returned type

A tupple which include the selected data and the indexes of selected data

```rust
pub type Returned<T> = (Vec<T>, Vec<usize>);
```

![valinta_demo](https://github.com/user-attachments/assets/d8637e6d-6c7a-4952-a22b-21fe03886473)
