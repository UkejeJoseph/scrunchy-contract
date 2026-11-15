#!/bin/bash
# Calculate estimated deployment costs for all Solana programs

set -e

echo "💰 Calculating Deployment Costs for Scrunchy Contracts"
echo "======================================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if programs are built
if [ ! -d "target/deploy" ]; then
    echo "❌ Programs not built yet. Run 'anchor build' first."
    exit 1
fi

# Get current balance
echo "📊 Current Wallet Status:"
BALANCE=$(solana balance 2>/dev/null | awk '{print $1}' || echo "0")
echo "   Current Balance: ${BLUE}$BALANCE SOL${NC}"
echo ""

# Program information
declare -A PROGRAMS=(
    ["access_control"]="n41USxoi9Lf1RyZUVAfCHD5TSeuDBuqDxEpUHzkDqf2"
    ["asset_registry"]="7vxpftgcdbh6NW77uAGVxp4aM2RcbQtmeGixNp2ifCDn"
    ["marketplace"]="G46EH6LskmJpBTvZNewSL4Xe5fkdbcdkMpRM8wQ9T3RH"
)

TOTAL_COST=0
TOTAL_SIZE=0
PROGRAM_COSTS=()

echo "📦 Program Details:"
echo "-------------------"

for program_name in "${!PROGRAMS[@]}"; do
    program_file="target/deploy/${program_name}.so"
    
    if [ ! -f "$program_file" ]; then
        echo "   ⚠️  ${program_name}.so - NOT FOUND"
        continue
    fi
    
    # Get file size in bytes
    SIZE_BYTES=$(stat -f%z "$program_file" 2>/dev/null || stat -c%s "$program_file" 2>/dev/null)
    SIZE_KB=$((SIZE_BYTES / 1024))
    SIZE_MB=$(echo "scale=2; $SIZE_BYTES / 1024 / 1024" | bc)
    
    # Calculate exact rent exemption using Solana CLI
    # Note: Program accounts need additional space for metadata, so we add ~10KB buffer
    ACCOUNT_SIZE=$((SIZE_BYTES + 10240))  # Program size + metadata overhead
    RENT_OUTPUT=$(solana rent $ACCOUNT_SIZE 2>/dev/null | grep "Rent-exempt minimum" | awk '{print $3}' || echo "")
    
    if [ -n "$RENT_OUTPUT" ]; then
        RENT_EXEMPTION="$RENT_OUTPUT"
    else
        # Fallback: approximate calculation
        # Solana program rent exemption is typically ~2.27-2.33 SOL for programs this size
        RENT_EXEMPTION=2.3
    fi
    
    # Transaction fee (approximate, can vary)
    TX_FEE=0.00176
    
    # Total cost per program
    PROGRAM_COST=$(echo "$RENT_EXEMPTION + $TX_FEE" | bc)
    TOTAL_COST=$(echo "$TOTAL_COST + $PROGRAM_COST" | bc)
    TOTAL_SIZE=$((TOTAL_SIZE + SIZE_BYTES))
    
    PROGRAM_COSTS+=("$program_name:$PROGRAM_COST")
    
    printf "   ${GREEN}✓${NC} %-20s | Size: %6s KB | Cost: ${YELLOW}%.5f SOL${NC}\n" \
        "$program_name" "$SIZE_KB" "$PROGRAM_COST"
done

echo ""
echo "📊 Summary:"
echo "-----------"
TOTAL_SIZE_KB=$((TOTAL_SIZE / 1024))
TOTAL_SIZE_MB=$(echo "scale=2; $TOTAL_SIZE / 1024 / 1024" | bc)

printf "   Total Programs:     ${BLUE}%d${NC}\n" "${#PROGRAMS[@]}"
printf "   Total Size:         ${BLUE}%d KB${NC} (%.2f MB)\n" "$TOTAL_SIZE_KB" "$TOTAL_SIZE_MB"
printf "   Estimated Cost:     ${YELLOW}%.5f SOL${NC}\n" "$TOTAL_COST"
echo ""

# Breakdown
echo "💵 Cost Breakdown (per program):"
echo "   • Program Rent Exemption: ~2.3 SOL"
echo "   • Transaction Fee:        ~0.00176 SOL"
echo "   • Total per program:      ~2.30176 SOL"
echo ""

# Check if balance is sufficient
REQUIRED=$(echo "$TOTAL_COST + 0.1" | bc)  # Add 0.1 SOL buffer
SUFFICIENT=$(echo "$BALANCE >= $REQUIRED" | bc)

if [ "$SUFFICIENT" -eq 1 ]; then
    echo "✅ ${GREEN}Sufficient balance!${NC} You have enough SOL to deploy all programs."
else
    NEEDED=$(echo "$REQUIRED - $BALANCE" | bc)
    echo "⚠️  ${YELLOW}Insufficient balance${NC}"
    printf "   You need: ${YELLOW}%.5f SOL${NC} more\n" "$NEEDED"
    echo ""
    echo "💡 To get more SOL on devnet:"
    echo "   solana airdrop 2"
    echo "   (Wait 10-15 minutes between requests due to rate limits)"
fi

echo ""
echo "🚀 To deploy all programs:"
echo "   anchor deploy"
echo ""

