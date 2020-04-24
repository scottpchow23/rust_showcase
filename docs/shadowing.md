# variable shadowing

## tl;dr

You can redeclare variables in rust which allows you to recreate the same variable with a different type. This is useful because you don't pollute the variable namespace for transformations on a variable while still maintaining type safety.

You can see a working example [here](../examples/shadowing/main.rs) and run it with `cargo run --example shadowing`.

## the details

In programming languages, [_variable shadowing_](https://en.wikipedia.org/wiki/Variable_shadowing) refers to the circumstance where a variable in an inner scope shares a name with another variable in an outer scope. Here it is demonstrated in Java and Python:

Java:

```java
public class Parent {
    // The original "x" variable
    static int x = 3;

    public static void main(String args[]) {
        // This local x "shadows" the static one above
        int x = 4;
        System.out.println("Local X is: " + x);
        System.out.println("Static X is:" + Parent.x);
        // Note: if this was an instance variable/method, you could use `this.field` to access it
    }
}
```

Python:

```python
# The original
x = 4

def block():
    # The shadow
    x = 5
    print(x)

# prints 5 because of scope
block()
# prints 4 because the shadow has gone out of scope
print(x)
```

What makes Rust interesting is that it can do variable shadowing without needing to change scope, all while maintaining type safety.

```rust
fn main() {
    let x = "5";
    println!("X is {}", x); // Prints `X is 5`
    let x = 3;
    println!("X is {}", x); // Prints `X is 3`
}
```

## how it works (conjecture)

Going off the observed behavior and this [answer](https://stackoverflow.com/questions/48227347/does-rust-free-up-the-memory-of-overwritten-variables) on stackoverflow, we believe that runtime-wise, variable shadowing is no more than simple variable hiding. That is to say that the alias used to refer to the old memory now refers to the new memory and the old memory is now inaccessible. This [snippet](https://users.rust-lang.org/t/newbie-question-memory-leaks-by-shadowing/9347/14) of C from the rust forums also provides the same idea:

```c
int main() {
    int x = 0;
    #define x x1
    int x = 1;
    printf("X is %d\n", x); // Will print 1 because of the #define changing x -> x1
}
```

## memory concerns

It should be noted that due to Rust's memory management system, all previously owned memory will remain allocated until the original owning reference is destroyed. In the previous example, the string `"5"` remains in scope until `main()` returns, despite no longer being accessible in any manner. This can be remedied by calling Rust's `drop()` function like so:

```rust
fn main() {
    let x = "5";
    drop(x); // original memory for "5" is now deallocated
    let x = 3;
    println!("X is {}", x);
} // memory for 3 is now deallocated
```

We also include example demonstrating dropping [here](../examples/drop/main.rs).

A more detailed explanation can be found on [here](https://stackoverflow.com/questions/48227347/does-rust-free-up-the-memory-of-overwritten-variables).
