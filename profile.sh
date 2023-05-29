#!/bin/bash -e

# Clone and update bytehound submodule
if [ ! -e "./bytehound/Cargo.toml" ]; then
    git submodule update --init --recursive
fi
# Build bytehound binaries from source if missing
if [ ! -e "./bytehound/target/release/libbytehound.so" ]; then
    cargo build --manifest-path ./bytehound/Cargo.toml --release -p bytehound-preload
fi
if [ ! -e "./bytehound/target/release/bytehound" ]; then
    cargo build --manifest-path ./bytehound/Cargo.toml --release -p bytehound-cli
fi

# Build the profiled binary
cargo build --release

# Run profiling
LD_PRELOAD=./bytehound/target/release/libbytehound.so RUST_LOG=info ./target/release/delta-rs-write-profiling $1

# Run server
./bytehound/target/release/bytehound server -p 8888 ./memory-profiling_*.dat
