use terminal_size::{Height, Width, terminal_size};

pub fn get_cols() -> u16 {
    let size = terminal_size().expect("Failed to get terminal size");
    if let (Width(w), Height(h)) = size {
        return w;
    } else {
        return 0;
    }
}
