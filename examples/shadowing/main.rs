use std::io;

fn main() {
    // Original declaration of the mutable variable "input"
    let mut input = String::new();
    println!(
        "input is currently \"{}\" (Quotes and trailing spaces not included).",
        input
    );

    println!("Input a number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // The redeclaration/shadow of the variable "input"
    let input: i32 = input.trim().parse().expect("Failed to parse a number");

    println!(
        "input is now \"{}\" (Quotes and trailing spaces not included).",
        input
    );
}
