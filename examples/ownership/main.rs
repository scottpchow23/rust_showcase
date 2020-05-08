//Taken from https://medium.com/@thomascountz/ownership-in-rust-part-1-112036b1126b

fn main() {
    let mut string = String::from("Hello, World!");
    println!("{:p}", string.as_ptr());
    foo(&mut string);
    println!("{:p}", string.as_ptr());
}

fn foo(string: &mut String) {
    println!("{:p}", string.as_ptr());
    string.push_str(".");
}
