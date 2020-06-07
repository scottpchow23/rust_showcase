# rust showcase

## the main program

The main program is a simple API fetcher; it retrieves the list of classes for Spring quarter 2020 from UCSB's [curriculumn api](https://developer.ucsb.edu/content/academic-curriculums) and serializes the results to the a file named `classes.json` in the root director of the project.

### setup

You'll need an API key from the UCSB API with the instructions [here](./docs/setup/README.md).

Assuming that is done and you have the rust toolchain installed (instructions [here](https://www.rust-lang.org/tools/install)), you can run the project with `cargo run`.

### points of interest

- A performance profile is created throughout the runtime of the program using the `flame` crate; the profile can be found in `flame-graph.html` after running the main program at least once.
- The classes can be retrieved in both a single and multi-threaded manner; both are profiled for comparison in the main file.
  - _Note:_ there is a difference in the latency of the first request as compared to all other requests; this is likely a result of the underlying use of TCP, which requires time to "warm up."

## examples

There are a few example programs in this repo that are used to illustrate concepts written about in [docs](./docs). You can get a list of examples to run by running:

```bash
cargo run --example
```

## feature documentation

Here is a list of features that we've looked into:

- [Shadowing](./docs/shadowing.md)
- [Ownership](./docs/ownership.md)
- [Types](./docs/types.md)
- [Channels](./docs/channels.md)
- [Debugging](./docs/debugging.md)
- [Profiling](./docs/profiling.md)

## benchmarking

### setup

In order to run the benchmark tests, you'll need to install the rust nightly toolchain.

If you don't already have rustup, the toolchain manager for rust, follow the instructions [here](https://rustup.rs/).

```bash
# Install nightly
rustup toolchain install nightly
```

You can run the benchmarks with

```bash
rustup run nightly cargo bench
```

### setting up benchmarks

- If you want benchmark tests, they are currently a nightly/unstable feature.
- If you want to use an unstable feature, you'll need to declare the flag at the top of the root file in your project (like line 1 of `main.rs` in this example).
- You then need to stick all of your tests in a module, and flag that for testing with `#[cfg(test)]`.
- Then flag all benchmarks with `#[bench]`.

### common benchmark pitfall

The Rust compiler is smart, and sometimes this can work against you if you're attempting to test the performance of something. See [this link](https://doc.rust-lang.org/1.12.1/book/benchmark-tests.html#gotcha-optimizations) for details on the common pitfall with regards to benchmarking performance.
