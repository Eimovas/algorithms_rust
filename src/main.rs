use crate::array_string::urification::urify;

mod array_string;
mod fibonacci;

fn main() {
    let mut str : Vec<char> =  "i am going for a walk                  ".chars().collect();
    let result = urify(&mut str, 21);

    let print1 : String = result.iter().collect();
    println!("{}", print1);
}
