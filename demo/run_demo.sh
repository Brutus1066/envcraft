#!/bin/bash
echo "============================================"
echo "  envcraft Demo - Precise tools for .env"
echo "  LazyFrog | kindware.dev"
echo "============================================"
echo

cd "$(dirname "$0")/.."

echo "[TEST 1] Check: Valid .env file"
echo "----------------------------------------"
cargo run --release -- check demo/schema.yml demo/valid.env
echo

echo "[TEST 2] Check: Invalid .env file (errors expected)"
echo "----------------------------------------"
cargo run --release -- check demo/schema.yml demo/invalid.env
echo

echo "[TEST 3] Diff: Compare dev vs prod"
echo "----------------------------------------"
cargo run --release -- diff demo/dev.env demo/prod.env
echo

echo "[TEST 4] Diff: With --redact flag (hide values)"
echo "----------------------------------------"
cargo run --release -- diff demo/dev.env demo/prod.env --redact
echo

echo "[TEST 5] Format: Show formatted output"
echo "----------------------------------------"
echo "Original messy.env:"
cat demo/messy.env
echo
echo "Formatted output:"
cargo run --release -- format demo/messy.env
echo

echo "============================================"
echo "  All demos complete!"
echo "============================================"
