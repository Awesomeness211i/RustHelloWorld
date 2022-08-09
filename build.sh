#! /bin/bash
cargo build --profile=dev
cargo build --quiet --profile=release
cargo run --quiet