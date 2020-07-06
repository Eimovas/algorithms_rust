/*
    Given a string, write a function to check if it is permutation of palindrome.
    You can ignore casing and non-letter chars.

    For example:
    input: Tact Coa
    output: true -> taco cat, atco cta
*/

use std::collections::HashMap;

pub fn is_palindrome_permutation(str : String) -> bool {
    // get char counts
    // if counts are all even, its true
    // else, see if only one odd count (then true), otherwise - false

    let mut counts: HashMap<char,i32> = HashMap::new();

    for char in str.chars() {
        let lower = char.to_ascii_lowercase();
        if lower.is_alphabetic() {
            if let Some(value) = counts.get(&lower) {
                counts.insert(lower, value + 1);
            }
            else {
                counts.insert(lower, 1);
            }
        }
    }

    let odd_count =  counts.values().filter(|v| *v % 2 != 0).count();
    odd_count == 0 || odd_count == 1
}