# types and lifetime specifiers

## types 101

Rust's type system manages two things: the type of a reference and it's lifetime. A type consists of the struct definition and the implmentation of it's methods, as shown below.

```rust
// The struct definition
pub struct Rectangle {
    pub height: f32,
    pub width: f32
}
// The implmentation
impl Point {
    // An instance method, as denoted by the leading `&self` parameter
    fn area(&self) -> f32 {
        height * width
    }
}
```

## traits

A Rust trait serves the same purpose as an interface in Java, which denotes _shared_ behavior across different structs.

```rust
pub trait PrettyPrint {
    fn prettyPrint(&self) -> String;
}
```

This trait can be implemented on a struct as follows:

```rust
impl PrettyPrint for Point {
    fn prettyPrint(&self) -> String {
        format!("self.height: {}\nself.width: {}", self.height, self.width)
    }
}
```

It should be noted that Rust has made a firm decision **not** to have classes, as the developers noted that class-based inheritance was often found to be unecessary or promoted excessive codesharing between classes and their subclasses. As a result, Rust deals with behavior sharing and polymorphism entirely through traits. A more complete explanation for this decision can be found in the Rust book [here](https://doc.rust-lang.org/book/ch17-01-what-is-oo.html#inheritance-as-a-type-system-and-as-code-sharing).

## generics

Another feature in modern programming languages is the ability to create functions that can take a generic type; that is to say, you only need to a method once for it to be able to apply the many methods.

Consider the following _non-generic_ function that identifies the largest float in a non-empty array of floats:

```rust
fn largest_f32(list: &[f32]) -> f32 {
    let mut largest = list[0];

    for &float in list {
        if float > largest {
            largest = float;
        }
    }
    largest
}
```

If I wanted a version of this function that would allow me to do the same thing, but for integers, I could copy it and use `i32` in place of `f32`. This could quickly get out of hand as there are a myriad of types for which I might want this function. Time to copy aside, it would also be particularly painful to go back and try to add in a check for if the list was empty _afterwards_.

And that's where generics come to the rescue: I can now declare the function with a single _generic_ type and have that serve for all of the types.

```rust
// This is almost it, but not quite
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

There are two lines of code that cause problems:

```rust
        // The use of the `>` operator is problem #1
        if item > largest {
            // The use of the `=` operator between two non-reference types is problem #2
            largest = item;
```

Simply put, I need generic types that have the `PartialOrd` and `Copy` traits to solve problems #1 and #2 respectively. As a result, the final iteration of this method looks like the following:

```rust
// This is almost it, but not quite
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

Note how there is room for a list of possible generic types between the function name and parameter list; we'll be working in that area a bit more in the next section.

In terms of performance concerns, Rust has got you covered. Instead of checking for compliance of types with the generics at runtime, Rust makes checking compliance with generic specifiers a job for the compiler. Furthermore, the compiler will compile a list of all types that use said generic, and will generate copies of the generic function for each specific type. For example, the following code:

```rust
let int_list = vec![1, 2, 3];
let largest_int = largest(int_list);
let float_list = vec![1.0, 2.0, 3.0];
let largest_float = largest(float_list);
```

Will end up having the following functions after compiling:

```rust
fn largest_i32(list: &[i32]) -> i32 { ... }
fn largest_f32(list: &[f32]) -> f32 { ... }
```

As such, Rust gives up a bit of space in order to not have any performance cost for using generics.

## lifetimes

One large problem that has yet to be addressed in our exploration of Rust is that of returning one of many references. Consider the following function that returns the shorter of two string _references_:

```rust
// This won't compile
fn shorter_string(str1: &str, str2: &str) -> &str {
    if (str1.len() < str2.len()) {
        str1
    } else {
        str2
    }
}
```

The problem is that Rust doesn't know what lifetime to assign to the return value of the function. You might think that it should just automatically give everything the same lifetime, but that won't always work. This is perhaps best displayed in the following snippet of code:

```rust
// This won't compile either
fn main() {
    let str1 = String.from("blah blah blah");
    let shortest;
    {
        let str2 = String.from("ha ha ha ha");
        shortest = shorter_string(str1.as_str(), str2.as_str());
    }
    println!("the shorter string is {}", shortest);
}
```

As we can see here, the two string variables have different lifetimes, with `str1` having the longer of the two. Also notice that the call to `println!` could potentially use a dead reference to `str2` whenever it is the shorter of the two strings and thus stored in `shortest`.

As a result, Rust will ask you to provide "lifetime annotations" for the various parameters and return values of a function. Lifetime annotation syntax is simply an extension on existing type annotations:

```rust
f32         // a float variable
&f32        // a reference to a float
&'a f32     // a reference to a float variable with lifetime 'a'
&'a mut f32 // a mutable reference to a float variable with lifetime 'a'
```

The point of lifetime annotations is to tell the compiler the relative lifetimes of parameters and return values. It should be stressed that the point of lifetime annotations is not to modify the lifetime of variables themselves, as those are determined by their scope.

In order to solve our earlier issue with `shorter_string`, we'll need to add lifetime annotations to explain to the compiler what relative lifetimes it should accept:

```rust
fn shorter_string<'a>(str1: &'a str, str2: &'a str) -> &'a str { ... }
```

These changes to the function declaration tell the compiler that for a given lifetime `'a`, it should accept variables that have lifetimes at least as long as `'a`. It also says that the function's return value will also live at least as long as `'a`. This, along with addressing the potential use of a dead reference in the earlier driver example should result in the following runnable driver:

```rust
fn main() {
    let str1 = String.from("blah blah blah");
    let shortest;
    {
        let str2 = String.from("ha ha ha ha");
        shortest = shorter_string(str1.as_str(), str2.as_str());
        println!("the shorter string is {}", shortest);
    }
}
```

One last note about lifetime and generics; lifetime annotations aren't generally used with generics because lifetime annotations are a part of the "generic-ness" of the generic type. That is to say `&i32` and `&'a i32` could both be captured under the same generic `T`.
