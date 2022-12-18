# Rust solutions

These are my Advent of Code 2022 solutions written in Rust.

For this to work, you need to download your input files in the top level `inputs` folder of the repository. They should be named like `day_01.txt`.

## Usage

Run the code:

```cli
# Run latest day
cargo run

# Run specific day
cargo run <day_number, e.g. 04>
```

Run with optimizations:

```cli
# Run latest day
cargo run --release

# Run specific day
cargo run <day_number, e.g. 04> --release
```

Run the tests:

```cli
cargo test
```

## Benchmarking

Run `./run_benchmark.sh`. This requires that `hyperfine` is installed.

## Tracing

To get a flamegraph and flamechart of the performance traces, run `./run_traced.sh`.

For this to work you need to run `cargo install inferno` once to install the flamechart tool.
