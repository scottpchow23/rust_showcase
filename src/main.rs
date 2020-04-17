#![feature(test)]
use std::io;

mod fib;

fn main() {
    let num = 40;
    println!("Hello, world!");
    println!("The {}th fibonacci number is {}", num, fib::fib_slow(num));
}
