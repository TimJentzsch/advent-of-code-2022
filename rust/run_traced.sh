cargo build --release --features traced
./target/release/rust
cat tracing.folded | inferno-flamegraph > tracing-flamegraph.svg
cat tracing.folded | inferno-flamegraph --flamechart > tracing-flamechart.svg
