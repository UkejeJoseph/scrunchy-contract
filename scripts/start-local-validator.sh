#!/bin/bash
# Start local Solana validator for unlimited airdrops and testing

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

PORT=${1:-8899}
LEDGER_DIR=${2:-.anchor/test-ledger}

echo ""
echo "🚀 Starting Local Solana Validator"
echo "==================================="
echo ""

# Check if validator is already running
if curl -s "http://127.0.0.1:${PORT}/health" > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Validator is already running on port ${PORT}${NC}"
    echo ""
    echo "To use it:"
    echo "  solana config set --url http://127.0.0.1:${PORT}"
    echo ""
    exit 0
fi

# Check if solana-test-validator is installed
if ! command -v solana-test-validator &> /dev/null; then
    echo -e "${RED}❌ solana-test-validator not found${NC}"
    echo ""
    echo "Install it with:"
    echo "  cargo install solana-test-validator"
    echo ""
    exit 1
fi

echo -e "${BLUE}ℹ️${NC} Starting validator on port ${PORT}..."
echo -e "${BLUE}ℹ️${NC} Ledger directory: ${LEDGER_DIR}"
echo ""

# Create ledger directory if it doesn't exist
mkdir -p "$LEDGER_DIR"

# Start validator
solana-test-validator \
    --reset \
    --port ${PORT} \
    --rpc-port ${PORT} \
    --faucet-port 9900 \
    --ledger "${LEDGER_DIR}" \
    --quiet &

VALIDATOR_PID=$!

echo -e "${GREEN}✅ Validator started (PID: ${VALIDATOR_PID})${NC}"
echo ""

# Wait for validator to be ready
echo "Waiting for validator to initialize..."
for i in {1..30}; do
    if curl -s "http://127.0.0.1:${PORT}/health" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Validator is ready!${NC}"
        echo ""
        echo "To use the local validator:"
        echo "  solana config set --url http://127.0.0.1:${PORT}"
        echo ""
        echo "To airdrop SOL (unlimited):"
        echo "  solana airdrop 1000"
        echo ""
        echo "To stop the validator:"
        echo "  kill ${VALIDATOR_PID}"
        echo "  # or"
        echo "  pkill solana-test-validator"
        echo ""
        exit 0
    fi
    sleep 1
    echo -n "."
done

echo ""
echo -e "${RED}❌ Validator failed to start${NC}"
echo "Check logs for errors"
exit 1

