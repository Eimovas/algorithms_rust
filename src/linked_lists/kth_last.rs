use std::collections::LinkedList;

#[allow(dead_code)]
pub fn find_kth_last_naive(list : LinkedList<i32>, k : usize) -> Option<i32> {
    // find length (if LL doesn't support length, iterate and get it)
    // calculate correct target index
    // iterate to index item and return
    let size = list.len();
    if size == 0 || k > size {
        return None;
    }


    let target_index = list.len() - k;
    for (i, &x) in list.iter().enumerate() {
        if i == target_index {
            return Some(x);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_list_when_empty_then_none() {
        let list = LinkedList::new();
        let result = find_kth_last_naive(list, 5);
        assert_eq!(None, result);
    }

    #[test]
    fn given_list_when_k_greater_than_length_then_none() {
        let mut list = LinkedList::new();
        list.push_front(1);

        let result = find_kth_last_naive(list, 5);
        assert_eq!(None, result);
    }

    #[test]
    fn given_list_when_k_zero_then_none() {
        let mut list = LinkedList::new();
        list.push_front(1);

        let result = find_kth_last_naive(list, 0);
        assert_eq!(None, result);
    }

    #[test]
    fn given_list_when_k_equals_list_length_then_return_head() {
        let mut list = LinkedList::new();
        list.push_front(1);

        let result = find_kth_last_naive(list, 1);
        assert_eq!(Some(1), result);
    }

    #[test]
    fn given_list_when_k_is_in_middle_then_return_valid() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        list.push_front(5);

        let result = find_kth_last_naive(list, 2);
        assert_eq!(Some(2), result);
    }
}