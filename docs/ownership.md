# ownership

## tl;dr

Each value in Rust has a variable that owns that value. When the variable goes out of scope, the value is garbage collected. Rust tries to reduce the number of duplicate objects created by not making copies which are expensive. 


## the details

The most basic example that highlights a difference is creating two variables that are equal to the same value. In Java and Python, the following code could be used:

Java:
```java
public class Foo {

    public static void main(String args[]) {
        int x = "4";
        int y = x;
        System.out.println("x is: " + x); //Prints 4
        System.out.println("y is: " + x); //Prints 4
    }
}
```

Python:
```python
x = "4"
y = x

print(x)    #Prints 4
print(y)    #Prints 4
```

The following code in rust achieves the same result:

Example 1:
```rust
fn main() {
    let x = "Hello, World!";
    let y = x;
    println!("{}", x);
}
```

However, similar code in Rust as shown below results in an error:

Example 2:
```rust
fn main() {
    let x = String::from("Hello, World!");
    let y = x;
    println!("{}", x);
}
//Error
//move occurs because `x` has type `std::string::String`, which does not implement the `Copy` trait
```

This is because when x in declared here, it is a String type that is being put onto the heap, instead of the previous example where it was being put onto the stack. Values on the heap in Rust belong to one variable, so when x is initially created, the string value "Hello, World!" belongs to x. But on the next line when y is declared, instead of creating a copy of x's value or letting y point to the value of x, Rust makes y have ownership of "Hello World!", and so now x has nothing. So when x is printed, it will not work. 

Rust does allow for cloning of dynamic data, in which case there will be 2 copies of the data on the stack. However, this is only possible for objects that implement the copy trait.

Example 3:
```rust
fn main() {
    let x = String::from("Hello, World!");
    let y = x.clone();
    println!("{}", x);
    println!("{}", y);
}

//move occurs because `x` has type `std::string::String`, which does not implement the `Copy` trait
```

So X and Y own their own copies of the string "Hello World". 

This system lets Rust prevent double free errors, where free is called on multiple variables that point to the same information. In example 2, when X and Y are garbage collected after the method returns, Rust already knows that X no longer points to the string "Hello World", and so this string will only be removed during the call to free Y. 

It is also possible for functions to pass ownership of variables back and forth. 

Example 4:
```rust 
fn main() {
    let string2 = foo();
    println!("{:p}", string2.as_ptr()); //0x21359b96bf0
}

fn foo() -> String {
    let string1 = String::from("Hello, World!");
    println!("{:p}", string1.as_ptr()); //0x21359b96bf0
    return string1;
}
```

In this example, string1 is created on the heap, however when the function foo returns, string2 is given ownership, as can be seen by their memory addresses. 

This can be taken one step further by both giving and taking ownership of variables. 

Example 5:

```rust 
fn main() {
    let string1 = String::from("Hello, World!");
    println!("{:p}", string1.as_ptr()); //0x1e7a1fa9c20
    let string3 = foo(string1);
    println!("{:p}", string3.as_ptr()); //0x1e7a1fa9c20
}

fn foo(string2: String) -> String {
    println!("{:p}", string2.as_ptr()); //0x1e7a1fa9c20
    return string2;
}
```

The string is created on the heap in main, and then passed to foo, and then passed back. Ownership is also being passed back and forth, so if foo didn't return "string2", then foo would be responsible for cleaning up the string when it returned. Instead main has this duty. 

However, a more common way of doing things in other languages is to create references. Rust allows for this too!

```rust 
fn main() {
    let string1 = String::from("Hello, World!");
    println!("{:p}", string1.as_ptr()); //0x1c8fcdbd130
    foo(&string1)
}

fn foo(string2: &String) -> String {
    println!("{:p}", string2.as_ptr()); //0x1c8fcdbd130
}
```

Main is passing a reference to the object on the heap to foo, however main does not pass ownership to foo. Instead, main is still responsible for cleaning up the string when it goes out of scope in main. Foo is only borrowing the pointer. Any number of these references can be created without changing the original ownership. 

However, this begs the question of what happens when a reference to the original object attempts to modify the object, as Rust does support mutability. The keyword 'mut' allows variables to be mutable. It is thus also possible to create a mutable reference. 

```rust
fn main() {
    let mut string = String::from("Hello, World!");
    println!("{:p}", string.as_ptr()); //0x2af69069c80
    foo(&mut string);
    println!("{:p}", string.as_ptr()); //0x2af6906de30
}

fn foo(string: &mut String) {
    println!("{:p}", string.as_ptr()); //0x2af69069c80
    string.push_str(".");
}
```

Main is still responsible for cleaning up the string and has sole ownership, but other functions are allowed to change the value. 

This raises an important question, what happens when multiple threads have mutable references to the same object? Rust solves this by making a stipulation that each object can have either one mutable reference or any number of immutable references. This completely prevents data races. 


Much of the code in this document is adapted from the following links:
https://medium.com/@thomascountz/ownership-in-rust-part-1-112036b1126b
https://medium.com/@thomascountz/ownership-in-rust-part-2-c3e1da89956e
