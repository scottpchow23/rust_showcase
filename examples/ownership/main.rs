//Taken from https://medium.com/@thomascountz/ownership-in-rust-part-1-112036b1126b

use std::io;

fn main() {
    let x = String::from("Hello, World!");
    let y = x.clone();
    println!("{}", x);
}
