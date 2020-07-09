use std::num;

fn main() {
    // there are three types of edits that can be performed on string - insert char, remove char, replace char.
    // given two strings, write a function if they are one edit (or zero edits) away from each other.
    // pale - ale -> true
    // pale - pal -> true
    // pale - pake -> true
    // pale - pakk -> false

    let input1 = String::from("pale");
    let input2 = String::from("pakk");
    let result = is_one_edit_away(input1, input2);

    println!("{}", result);
}

pub fn is_one_edit_away(str1: String, str2: String) -> bool {
    let size_diff = (str1.len() - str2.len()) as i32;
    if size_diff.abs() > 1 {
        return false;
    }

    match size_diff == 0 {
        true => is_one_edit_equal_str_length(str1, str2),
        false => is_one_edit_not_equal_str_length(str1, str2)
    }
}

fn is_one_edit_equal_str_length(str1: String, str2: String) -> bool {
    let str1: Vec<char> = str1.chars().collect();
    let str2: Vec<char> = str2.chars().collect();

    let mut edited = false;
    for index in 0..str1.len() {
        if edited == false && str1[index] != str2[index] {
            edited = true;
        } else if edited == true && str1[index] != str2[index] {
            return false;
        } else {
            continue;
        }
    }
    return true;
}

fn is_one_edit_not_equal_str_length(str1: String, str2: String) -> bool {
    // pale - ale
    // pale - pal
    // if insert is needed, edit can't happen anymore - that would be 2 edits
    // meaning if letter doesn't match in longer string, i can move longer index + 1 and do comparison again -> it has to match perfect onwards
    // if i move index and that exceeds longer length (and no inserts happened before) -> it goes true

    let (longer, shorter) = if str1.len() > str2.len() { (str1, str2) } else { (str2, str1) };
    let longer: Vec<char> = longer.chars().collect();
    let shorter: Vec<char> = shorter.chars().collect();

    let mut edited = false;
    let mut short_index = 0;
    for long_index in 0..longer.len() {
        if short_index == shorter.len() {
            // when last char of longer str, and edit hasn't happen before
            return edited == false;
        }
        if edited == false && longer[long_index] != shorter[short_index] {
            edited = true;
        } else if edited == true && longer[long_index] != shorter[short_index] {
            return false;
        } else {
            short_index += 1;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_strings_when_one_replace_apart_then_true() {
        let input1 = String::from("pale");
        let input2 = String::from("pall");
        assert_eq!(true, is_one_edit_away(input1, input2));
    }

    #[test]
    fn given_strings_when_one_insert_at_start_apart_then_true() {
        let input1 = String::from("pale");
        let input2 = String::from("ale");
        assert_eq!(true, is_one_edit_away(input1, input2));
    }

    #[test]
    fn given_strings_when_one_insert_at_end_apart_then_true() {
        let input1 = String::from("pale");
        let input2 = String::from("pal");
        assert_eq!(true, is_one_edit_away(input1, input2));
    }

    #[test]
    fn given_strings_when_two_edits_apart_then_false() {
        let input1 = String::from("pale");
        let input2 = String::from("pakk");
        assert_eq!(false, is_one_edit_away(input1, input2));
    }
}