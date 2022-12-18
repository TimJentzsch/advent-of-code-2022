cargo build --release
hyperfine --warmup 10 "./target/release/rust"
