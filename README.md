# rust showcase

## examples

There are a few example programs in this repo that are used to illustrate concepts written about in [docs](./docs). You can get a list of examples to run by running:

```bash
cargo run --example
```

## benchmarking

### setup

In order to run the benchmark tests, you'll need to install the rust nightly toolchain.

If you don't already have rustup, the toolchain manager for rust, follow the instructions [here](https://rustup.rs/).

```bash
# Install nightly
rustup toolchain install nightly
```

Run the benchmarks with

```bash
rustup run nightly cargo bench
```

You'll also need an API key from the UCSB API with the instructions [here](./docs/setup/README.md).

### setting up benchmarks

- If you want benchmark tests, they are currently a nightly/unstable feature.
- If you want to use an unstable feature, you'll need to declare the flag at the top of the root file in your project (like line 1 of `main.rs` in this example).
- You then need to stick all of your tests in a module, and flag that for testing with `#[cfg(test)]`.
- Then flag all benchmarks with `#[bench]`.

### writing benchmarks

Rust's compiler is smart, and sometimes this can work against you if you're attempting to test the performance of something. See [this link](https://doc.rust-lang.org/1.12.1/book/benchmark-tests.html#gotcha-optimizations) for details on the common pitfall with regards to benchmarking performance.
