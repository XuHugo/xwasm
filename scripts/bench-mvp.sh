#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
transactions="${1:-100}"
concurrency="${2:-4}"
mode="${3:-all}"
contract_wasm="$repo_root/contract/target/wasm32-unknown-unknown/release/contract.wasm"

rustup target add wasm32-unknown-unknown
cargo build --manifest-path "$repo_root/contract/Cargo.toml" --target wasm32-unknown-unknown --release

run_benchmark() {
  XWASM_CONTRACT_WASM="$contract_wasm" cargo run --release --manifest-path "$repo_root/par-wasm/Cargo.toml" --example benchmark -- "$transactions" "$concurrency" "$1"
}

case "$mode" in
  all)
    run_benchmark compute
    run_benchmark independent
    run_benchmark hotspot
    ;;
  independent|hotspot|compute)
    run_benchmark "$mode"
    ;;
  *)
    echo "mode must be all, independent, hotspot, or compute" >&2
    exit 2
    ;;
esac
