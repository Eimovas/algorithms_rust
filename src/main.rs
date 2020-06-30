use crate::array_string::is_permutation::{is_permutation_array, is_permutation_sort, is_permutation_hash};

mod array_string;
mod fibonacci;

fn main() {
    let str1 = "jonas";
    let str2 = "sanoj";
    let result = is_permutation_hash(&str1, &str2);

    println!("{}", result);
}
