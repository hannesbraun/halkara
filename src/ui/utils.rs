use terminal_size::{terminal_size, Width};

pub fn term_width() -> u16 {
    let term_size = terminal_size();
    if let Some((Width(w), _)) = term_size {
        w
    } else {
        80
    }
}
