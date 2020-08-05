// You have two numbers represented by a linked list, where each node contains a single digit. The digits are stored
// in a reverse order. Write function that adds the two numbers and returns the sum as a reversed linked list.
//
// Example:
// Input: (7 - 1 - 6) + (5 - 9 - 2). That's 617+295
// Output: 2 - 1 - 9. That's 912.

use std::collections::LinkedList;

#[allow(dead_code)]
pub fn add_lists(first : &mut LinkedList<char>, second : &mut LinkedList<char>) -> LinkedList<char> {
    let mut remainder : u8 = 0;
    let mut digits = vec![];
    loop {
        let x = first.pop_front();
        let y = second.pop_front();

        match (x,y) {
            (Some(x), Some(y)) => {
                let (sum,remain) = basic_summation(x.to_digit(10).unwrap() as u8, y.to_digit(10).unwrap() as u8, remainder);
                remainder = remain;
                digits.push(sum);
            },
            (Some(x), None) => {
                let (sum,remain) = basic_summation(x.to_digit(10).unwrap() as u8, 0, remainder);
                remainder = remain;
                digits.push(sum);
            },
            (None, Some(x)) => {
                let (sum,remain) = basic_summation(x.to_digit(10).unwrap() as u8, 0, remainder);
                remainder = remain;
                digits.push(sum);
            },
            (None, None) => {
                break
            }
        }
    }
    let mut result = LinkedList::new();
    for i in digits {
        result.push_back(i);
    }
    result
}

fn basic_summation(x : u8, y : u8, remainder : u8) -> (char,u8) {
    let sum = x + y;
    if sum > 9 {
        ((sum % 10 + 48 + remainder) as char, 1)
    }
    else {
        ((sum + remainder + 48) as char, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn given_lists_when_both_empty_then_return_empty() {
        let mut first = LinkedList::new();
        let mut second = LinkedList::new();
        let result = add_lists(&mut first, &mut second);
        assert_eq!(0, result.len());
    }
    #[test]
    fn given_lists_when_first_is_empty_then_return_valid() {
        let mut first = LinkedList::new();
        let mut second = LinkedList::new();
        second.push_front('1');
        second.push_front('2');

        let result : Vec<char> = add_lists(&mut first, &mut second).iter().map(|x| *x).collect();
        assert_eq!(result, vec!['2','1']);
    }

    #[test]
    fn given_lists_when_second_is_empty_then_return_valid() {
        let mut first = LinkedList::new();
        let mut second = LinkedList::new();

        first.push_front('1');
        first.push_front('2');

        let result : Vec<char> = add_lists(&mut first, &mut second).iter().map(|x| *x).collect();
        assert_eq!(result, vec!['2','1']);
    }

    #[test]
    fn given_lists_when_not_empty_then_return_valid() {
        let mut first = LinkedList::new();
        let mut second = LinkedList::new();

        first.push_front('6');
        first.push_front('1');
        first.push_front('7');

        second.push_front('2');
        second.push_front('9');
        second.push_front('5');

        let result : Vec<char> = add_lists(&mut first, &mut second).iter().map(|x| *x).collect();
        assert_eq!(result, vec!['2','1', '9']);
    }
}