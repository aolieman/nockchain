#!/bin/bash
set -euo pipefail

# This script builds the fakenet miner and runs it.
# Run from ~/nockchain
# ./scripts/fake_miner.sh [--hoon] |& tee "fake-miner/logs/fake-$(date +%s).log"

# Only build if explicitly requested
if [[ "${1:-}" == "--hoon" ]]; then
  make fakenet-assets
fi

# Run the fakenet miner
cd fake-miner || { echo "Missing fake-miner dir"; exit 1; }
cargo run --manifest-path ../Cargo.toml --release -p nockchain --target-dir ../target-fake -- \
    --fakenet \
    --genesis-leader \
    --mining-pubkey $MINING_PUBKEY \
    --mine \
    --new-peer-id \
    --no-default-peers
