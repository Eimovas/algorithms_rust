mod unique_chars;
mod fibonacci;

use crate::unique_chars::{has_unique_chars, has_unique_chars_hash};

fn main() {
    let result = has_unique_chars_hash("new");
    println!("{}", result);

    let result = has_unique_chars_hash("not unique");
    println!("{}", result);
}
