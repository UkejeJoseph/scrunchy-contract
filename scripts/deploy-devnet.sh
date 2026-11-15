#!/bin/bash
# Deploy Scrunchy contracts to Solana devnet

set -e

echo "🚀 Deploying Scrunchy Contracts to Devnet"
echo "=========================================="
echo ""

# Check Solana config
echo "📋 Checking Solana configuration..."
RPC_URL=$(solana config get | grep "RPC URL" | awk '{print $3}')
if [[ "$RPC_URL" != *"devnet"* ]]; then
    echo "⚠️  Warning: RPC URL doesn't appear to be devnet: $RPC_URL"
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check balance
echo "💰 Checking wallet balance..."
BALANCE=$(solana balance | awk '{print $1}')
echo "   Balance: $BALANCE SOL"

if (( $(echo "$BALANCE < 1" | bc -l) )); then
    echo "⚠️  Low balance. Airdropping 2 SOL..."
    solana airdrop 2
    sleep 2
    solana balance
fi

# Build programs
echo ""
echo "🔨 Building programs..."
anchor build

# Verify .so files exist
echo ""
echo "✅ Verifying compiled programs..."
for program in access_control asset_registry marketplace; do
    if [ -f "target/deploy/${program}.so" ]; then
        SIZE=$(ls -lh "target/deploy/${program}.so" | awk '{print $5}')
        echo "   ✓ ${program}.so ($SIZE)"
    else
        echo "   ✗ ${program}.so - NOT FOUND"
        exit 1
    fi
done

# Deploy
echo ""
echo "🚀 Deploying to devnet..."
anchor deploy

# Verify deployment
echo ""
echo "🔍 Verifying deployment..."
echo ""
PROGRAMS=(
    "n41USxoi9Lf1RyZUVAfCHD5TSeuDBuqDxEpUHzkDqf2:access_control"
    "7vxpftgcdbh6NW77uAGVxp4aM2RcbQtmeGixNp2ifCDn:asset_registry"
    "G46EH6LskmJpBTvZNewSL4Xe5fkdbcdkMpRM8wQ9T3RH:marketplace"
)

for program_info in "${PROGRAMS[@]}"; do
    IFS=':' read -r program_id program_name <<< "$program_info"
    echo "Checking $program_name..."
    if solana program show "$program_id" > /dev/null 2>&1; then
        echo "   ✅ $program_name deployed successfully"
        echo "   🔗 https://explorer.solana.com/address/$program_id?cluster=devnet"
    else
        echo "   ❌ $program_name deployment failed"
    fi
    echo ""
done

echo "✅ Deployment complete!"
echo ""
echo "📝 Next steps:"
echo "   1. Update backend with deployed program IDs"
echo "   2. Test with devnet transactions"
echo "   3. Verify on Solana Explorer"

