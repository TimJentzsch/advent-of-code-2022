cargo run --release --features traced
cat tracing.folded | inferno-flamegraph > tracing-flamegraph.svg
cat tracing.folded | inferno-flamegraph --flamechart > tracing-flamechart.svg
