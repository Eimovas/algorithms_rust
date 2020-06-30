use std::collections::{HashSet, HashMap};

// assuming ASCII char set of strings
pub fn is_permutation_array(str1 : &str, str2 : &str) -> bool {
    if str1.len() != str2.len() {
        return false;
    }

    let mut arr = [0; 128];
    for c in str1.chars() {
        let index = c as usize;
        arr[index] += 1;
    }

    for c in str2.chars() {
        let index = c as usize;
        arr[index] -= 1;
        if arr[index] < 0 {
            return false;
        }
    }

    return true;
}

pub fn is_permutation_sort(str1 : &str, str2 : &str) -> bool {
    if str1.len() != str2.len() {
        return false;
    }

    let mut sorted1 : Vec<char> = str1.chars().collect();
    sorted1.sort();

    let mut sorted2 : Vec<char> = str2.chars().collect();
    sorted2.sort();

    sorted1 == sorted2
}

pub fn is_permutation_hash(str1 : &str, str2 : &str) -> bool {
    if str1.len() != str2.len() {
        return false;
    }

    let mut hash_map1 : HashMap<char,i32> = HashMap::new();
    for c in str1.chars() {
        let mut value = hash_map1.entry(c).or_insert(0);
        *value += 1;
    }

    let mut hash_map2 : HashMap<char,i32> = HashMap::new();
    for c in str2.chars() {
        let mut value = hash_map2.entry(c).or_insert(0);
        *value += 1;
    }

    for (k,v) in hash_map1 {
        match hash_map2.get(&k) {
            Some(x) => {
                if x != &v {
                    return false;
                }
                else {
                    continue;
                }
            },
            None => {
                return false;
            }
        }
    }

    return true;
}