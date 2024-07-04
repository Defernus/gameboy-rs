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

fetch-opcodes-tests:
    rm -rf opcode-tests-repo cpu-test-data
    git clone https://github.com/adtennant/GameboyCPUTests.git opcode-tests-repo
    cd opcode-tests-repo && git checkout d1c8fd48c9e543995397fbd20e55bcbd524d029f && cd ..
    mkdir -p cpu-test-data
    mv opcode-tests-repo/v2/* cpu-test-data/
    rm -rf opcode-tests-repo

test:
    cargo nextest run --run-ignored default

test-integration:
    cargo nextest run --run-ignored ignored-only

test-all:
    cargo nextest run --run-ignored all