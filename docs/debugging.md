# debugging

Credit to Bryce Fisher-Fleig and his [article](http://bryce.fisher-fleig.org/blog/debugging-rust-programs-with-lldb/index.html) on `rust-lldb`.

As we've learned, there's no such distinction between a compiled and an interpreted language; it's a question of the runtime you end up _choosing_ to run your code in.

That being said, Rust is a systems language and as a result is generally **compiled**.

That led to an interesting question: what does debugging Rust look like? A quick google search currently (4/30/2020) reveals nothing resembling a Rust specific debugger, but there's no way that there is no way to debug a systems level language with only `println!`, right?

## xxdb to the rescue

This was quite an ephiphany for me: for compiled languages, there is generally no language specific debugger but instead an assembly specific debugger. That is to say, if I compile Rust to an x86 binary, then I can load and run Rust with my favorite C/C++ debugger. In this example I'll be using `lldb` as my debugger of choice but known that the same principles apply to other debuggers like `gdb`.

So in theory, we should just be able to take an executable (compiled with symbols) and just feed it to `lldb` right? Let's try debugging our shadowing executable, found under `./target/debug/examples/shadowing`:

```bash
# compile
cargo build --examples
# debug
lldb ./target/debug/examples/shadowing
```

We're greeted with the following:

```
(lldb) target create "target/debug/examples/shadowing"
Current executable set to '/Users/scottpchow/Stuff/grad/programming_runtime_263/test_showcase/target/debug/examples/shadowing' (x86_64).
```

So it loads! Let's quickly set a breakpoint on `main` by running `b main` and start the program with `r`.

```
(lldb) b main
Breakpoint 1: 2 locations.
(lldb) r
Process 91882 launched: '/test_showcase/target/debug/examples/shadowing' (x86_64)
Process 91882 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.2
    frame #0: 0x0000000100001610 shadowing`main
shadowing`main:
->  0x100001610 <+0>: pushq  %rbp
    0x100001611 <+1>: movq   %rsp, %rbp
    0x100001614 <+4>: subq   $0x10, %rsp
    0x100001618 <+8>: movslq %edi, %rax
Target 0: (shadowing) stopped.
```

Interesting; it seems like there is some outer wrapper to the Rust program that is also called `main`. You can read more about this shim method [here](https://stackoverflow.com/questions/38416394/unable-to-set-a-breakpoint-on-main-while-debugging-a-program-compiled-with-rust), but for now we'll move past it using `c`.

```
(lldb) c
Process 91882 resuming
Process 91882 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
    frame #0: 0x000000010000134b shadowing`shadowing::main::h920c14c2e70fdc79 at main.rs:5:20
   2
   3    fn main() {
   4        // Original declaration of the mutable variable "input"
-> 5        let mut input = String::new();
   6        println!(
   7            "input is currently \"{}\" (Quotes and trailing spaces not included).",
   8            input
Target 0: (shadowing) stopped.
```

Look at that! `lldb` is able to link the binary's instructions to lines in the source code file (courtesy of debug symbols). Using `n`, let's step through one line so that `input` becomes initialized and take a look at the input variable using `p input`.

```
(lldb) n
Process 91882 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = step over
    frame #0: 0x0000000100001364 shadowing`shadowing::main::h920c14c2e70fdc79 at main.rs:7:8
   4        // Original declaration of the mutable variable "input"
   5        let mut input = String::new();
   6        println!(
-> 7            "input is currently \"{}\" (Quotes and trailing spaces not included).",
   8            input
   9        );
   10
Target 0: (shadowing) stopped.
(lldb) p input
(alloc::string::String) input = {
  vec = {
    buf = {
      ptr = (pointer = "", _marker = core::marker::PhantomData<unsigned char> @ 0x00007ffeefbff060)
      cap = 0
      a = {}
    }
    len = 0
  }
}
```

Okay, that's cool; we have all of the information about `input` as well as a peek under the covers as to how `String` is implemented.

## rust-xxdb

It should be noted that there are extensions `rust-gdb` and `rust-lldb` that are simply Python wrappers around `gdb` and `lldb` that print Rust debug information in a more friendly format. They come installed with the `rustup` toolchain and can be run exactly like `gdb` and `lldb`. Start it with `rust-lldb ./target/debug/examples/shadowing`, follow the same steps above, and you should get this output instead:

```
(lldb) p input
(alloc::string::String) $0 = ""
```

## limitations of xxdb debuggers

The primary shortfall with `rust-gdb` and `rust-lldb` is expression parsing/execution. That is to say, they can't parse/execute _Rust_ expressions; it will still parse/execute C/C++ expressions however. Adding parsing/execution of Rust expressions would require extending the `gdb` and `lldb` codebases directly.

It should be noted that if you're allowed to execute arbitrary C/C++ expressions, it seems like it would be possible to violate Rust compile-time constraints. One example might be forcibly updating the underlying reference of a variable to point to another variable; this could violate Rust's ownership principle of having only one owner per variable.
