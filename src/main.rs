use crate::array_string::palindrome_permutation::is_palindrome_permutation;

mod linked_lists;
mod array_string;
mod fibonacci;

fn main() {
    let input = String::from("atco cta");
    let result = is_palindrome_permutation(input);

    println!("{}", result);
}
