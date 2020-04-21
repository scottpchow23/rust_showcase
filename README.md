# rust showcase

In order to run the tests, you'll need to install the rust nightly toolchain.

If you don't already have rustup, the toolchain manager for rust, follow the instructions [here](https://rustup.rs/).

```bash
# Install nightly
rustup toolchain install nightly
```

Run the benchmarks with

```bash
rustup run nightly cargo bench
```

## setting up benchmarks

- If you want benchmark tests, they are currently a nightly/unstable feature.
- If you want to use an unstable feature, you'll need to declare the flag at the top of the root file in your project (like line 1 of `main.rs` in this example).
- You then need to stick all of your tests in a module, and flag that for testing with `#[cfg(test)]`.
- Then flag all benchmarks with `#[bench]`.

## writing benchmarks

Rust's compiler is smart, and sometimes this can work against you if you're attempting to test the performance of something. See [this link](https://doc.rust-lang.org/1.12.1/book/benchmark-tests.html#gotcha-optimizations) for details on the common pitfall with regards to benchmarking performance.
