set export
set dotenv-load

default:
    @just --list

fmt:
    cargo fmt --all

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

run:
    cargo run --bin runner
