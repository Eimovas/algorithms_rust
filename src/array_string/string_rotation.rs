#[allow(dead_code)]
fn is_substring(str1 : String, str2 : String) -> bool {
    str2.contains(str1.as_str())
}

pub fn is_rotation(str1 : String, str2 : String) -> bool {
    let str = format!("{}{}", str1, str1);
    is_substring(str2, str)
}

#[cfg(test)]
mod tests {
    use crate::array_string::string_rotation::is_rotation;

    #[test]
    fn given_strings_when_valid_rotation_then_return_true() {
        let str1 = String::from("erbottlewat");
        let str2 = String::from("waterbottle");

        assert_eq!(true, is_rotation(str1, str2));
    }

    #[test]
    fn given_strings_when_not_valid_rotation_then_return_false() {
        let str1 = String::from("erbottlewatt");
        let str2 = String::from("waterbottle");

        assert_eq!(false, is_rotation(str1, str2));
    }
}