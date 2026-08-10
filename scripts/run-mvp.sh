#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
contract_wasm="$repo_root/contract/target/wasm32-unknown-unknown/release/contract.wasm"

rustup target add wasm32-unknown-unknown
cargo build --manifest-path "$repo_root/contract/Cargo.toml" --target wasm32-unknown-unknown --release

XWASM_CONTRACT_WASM="$contract_wasm" cargo test --manifest-path "$repo_root/par-wasm/Cargo.toml" --test parallel executes_real_edsl_wasm_commits_and_recovers_state -- --exact --nocapture
