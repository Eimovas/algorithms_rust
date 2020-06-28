
// Write an algorithm to determine if string has all unique characters. Try not to use any data structures.

use std::collections::HashSet;

// assuming str is ASCII
pub fn has_unique_chars(str : String) -> bool {
    let mut arr = [false;128];
    for c in str.chars() {
        let num = c as usize;

        if arr[num] == true {
            return false;
        }
        else {
            arr[num] = true;
        }
    }

    true
}

pub fn has_unique_chars_hash(str : &str) -> bool {
    let mut set : HashSet<char> = HashSet::new();
    for c in str.chars() {
        if set.contains(&c) {
            return false;
        }
        else {
            set.insert(c);
        }
    }

    true
}