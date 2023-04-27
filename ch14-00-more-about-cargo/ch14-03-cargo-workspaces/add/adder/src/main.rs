extern crate add_one;
extern crate add_two;
extern crate rand;

fn main() {
    println!("Hello, world!");
    println!("1 + 2 = {}", add_one::add_one(2));
    println!("2 + 3 = {}", add_two::add_two(3));
}
