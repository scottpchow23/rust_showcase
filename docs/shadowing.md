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
    let x = 3;
    println!("X is {}", x);
}
```
