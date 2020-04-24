# ownership

## tl;dr

Each value in Rust has a variable that owns that value. When the variable goes out of scope, the value is 


## the details

The most basic example the highlights a difference is creating two variables that are equal to the same value. In Java and Python, the following code could be used:

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

```python
x = "4"
y = x

print(x)    #Prints 4
print(y)    #Prints 4
```

The following code in rust achieves the same result:

```rust
fn main() {
    let x = "Hello, World!";
    let y = x;
    println!("{}", x);
}
```

However, similar code in Rust as shown below results in an error:

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

Rust does allow for cloning of dynamic data, in which case there will be 2 copies of the data on the stack, as seen in the below example:

```rust
fn main() {
    let x = String::from("Hello, World!");
    let y = x.clone();
    println!("{}", x);
}

//move occurs because `x` has type `std::string::String`, which does not implement the `Copy` trait
```