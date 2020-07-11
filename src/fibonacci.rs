#[allow(dead_code)]
pub fn get_nth(n : u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut previous1 = 0;
    let mut previous2 = 1;

    for _ in 2..n+1 {
        let current = previous1 + previous2;
        previous1 = previous2;
        previous2 = current;
    }

    previous2
}