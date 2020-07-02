// 3- encode empty spaces in given string with %20. You may assume that string has sufficient space for this.
// You are given a 'true' length of string (the length of actual text with spaces).

fn get_true_space_count(chars : &[char]) -> usize {
    let mut spaces = 0;
    for c in chars {
        if *c == ' ' {
            spaces += 1;
        }
    }
    spaces
}

pub fn urify(chars : &mut Vec<char>, true_length : usize) -> &Vec<char> {
    let true_space_count = get_true_space_count(&chars[0..true_length]);
    let mut new_index= true_length - 1 + true_space_count * 2;

    for i in (1..true_length).rev() {
        let index = i as usize;

        if chars[index] == ' ' {
            chars[new_index] = '0';
            chars[new_index-1] = '2';
            chars[new_index-2] = '%';
            new_index -= 3;
        }
        else {
            chars[new_index] = chars[index];
            new_index -=1;
        }
    }
    chars
}