extern crate fudd;
extern crate lazy_static;
use std::io::stdin;

use fudd::get_fudd;

fn main() {
    println!("Hello please type something in");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    /*    lazy_static! {
        static ref FUDD_REGEX: Regex = Regex::new(hm.keys().collect()).unwrap();
    }*/
    let unfudded = input.trim();
    let result = get_fudd(unfudded);
    println!("{}", result)
}
