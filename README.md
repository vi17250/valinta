# [x] Valinta

> _Valinta_ a zero config Rust crate 🦀 for multiple selection in the terminal

# 🎞️ Example

![valinta-demo](https://github.com/user-attachments/assets/3e948218-79ca-4ee6-bbbf-11d69c5261f8)

# 📚 Usage

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

# 👩‍💻 User interactions

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

# 🪇 Returned type

A tupple which include the selected items and the indexes of selected data

```rust
pub type Returned<T> = (Vec<T>, Vec<usize>);
```
