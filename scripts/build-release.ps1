$ErrorActionPreference = "Stop"

cargo build --release --all-features
cargo test --release --all-features
