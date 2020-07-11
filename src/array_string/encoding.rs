use std::fmt::Write;

#[allow(dead_code)]
pub fn encode(str : String) -> String {
    if str.len() == 0 {
        return str;
    }

    let mut chars = str.chars();
    let mut previous_char = chars.next().unwrap();
    let mut current_count = 1;
    let mut result = String::new();

    for char in chars {
        if char == previous_char {
            current_count += 1;
        } else {
            write!(&mut result, "{}{}", previous_char, current_count).unwrap();
            previous_char = char;
            current_count = 1;
        }
    }

    write!(&mut result, "{}{}", previous_char, current_count).unwrap();

    if result.len() >= str.len() {
        str
    } else {
        result
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_string_when_empty_then_return_empty() {
        let input = String::new();
        assert_eq!(String::new(), encode(input));
    }

    #[test]
    fn given_string_when_single_letter_then_return_original() {
        let input = String::from("a");
        assert_eq!(String::from("a"), encode(input));
    }

    #[test]
    fn given_string_when_repeating_chars_upfront_then_return_encoded() {
        let input = String::from("aaaaaaab");
        assert_eq!(String::from("a7b1"), encode(input));
    }

    #[test]
    fn given_string_when_repeating_chars_in_back_then_return_encoded() {
        let input = String::from("baaaaaaa");
        assert_eq!(String::from("b1a7"), encode(input));
    }

    #[test]
    fn given_string_when_casing_mixed_then_return_encoded() {
        let input = String::from("aaaaaaabbbbbbAAAAA");
        assert_eq!(String::from("a7b6A5"), encode(input));
    }
}
