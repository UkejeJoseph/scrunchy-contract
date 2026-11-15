#!/bin/bash
# Deploy programs one at a time to avoid RPC issues

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo ""
echo "🚀 Individual Program Deployment Script"
echo "========================================"
echo ""

# Check balance
BALANCE=$(solana balance | awk '{print $1}')
echo -e "${BLUE}ℹ️${NC} Current balance: ${BALANCE} SOL"
echo ""

# Programs to deploy (in order of size, smallest first)
PROGRAMS=("access_control" "asset_registry" "marketplace")

for program in "${PROGRAMS[@]}"; do
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Deploying: ${program}${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    # Check if program file exists
    if [ ! -f "target/deploy/${program}.so" ]; then
        echo -e "${RED}❌ Program file not found: target/deploy/${program}.so${NC}"
        echo "   Run 'anchor build' first"
        exit 1
    fi
    
    # Show program size
    SIZE=$(ls -lh "target/deploy/${program}.so" | awk '{print $5}')
    echo -e "${BLUE}ℹ️${NC} Program size: ${SIZE}"
    echo ""
    
    # Try deployment with retries
    MAX_RETRIES=3
    RETRY_COUNT=0
    SUCCESS=false
    
    while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
        if [ $RETRY_COUNT -gt 0 ]; then
            echo -e "${YELLOW}⚠️${NC} Retry attempt ${RETRY_COUNT}/${MAX_RETRIES}..."
            echo -e "${BLUE}ℹ️${NC} Waiting 10 seconds before retry..."
            sleep 10
        fi
        
        echo -e "${BLUE}ℹ️${NC} Deploying ${program}..."
        
        if anchor deploy --program-name "${program}" 2>&1 | tee /tmp/deploy-${program}.log; then
            echo ""
            echo -e "${GREEN}✅ Successfully deployed ${program}!${NC}"
            SUCCESS=true
            break
        else
            RETRY_COUNT=$((RETRY_COUNT + 1))
            echo ""
            echo -e "${RED}❌ Deployment failed${NC}"
            
            # Check for specific errors
            if grep -q "216 write transactions failed" /tmp/deploy-${program}.log; then
                echo -e "${YELLOW}⚠️${NC} RPC endpoint issue detected"
                echo -e "${BLUE}ℹ️${NC} This is usually due to network congestion or RPC rate limits"
            fi
            
            if [ $RETRY_COUNT -lt $MAX_RETRIES ]; then
                echo -e "${BLUE}ℹ️${NC} Will retry..."
            fi
        fi
    done
    
    if [ "$SUCCESS" = false ]; then
        echo ""
        echo -e "${RED}❌ Failed to deploy ${program} after ${MAX_RETRIES} attempts${NC}"
        echo ""
        echo -e "${YELLOW}💡 Suggestions:${NC}"
        echo "   1. Wait a few minutes and try again"
        echo "   2. Check your internet connection"
        echo "   3. Try a different RPC endpoint"
        echo "   4. Check if devnet is experiencing issues"
        echo ""
        exit 1
    fi
    
    echo ""
    echo -e "${BLUE}ℹ️${NC} Waiting 5 seconds before next deployment..."
    sleep 5
    echo ""
done

echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ All programs deployed successfully!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Verify deployments
echo -e "${BLUE}ℹ️${NC} Verifying deployments..."
echo ""

for program in "${PROGRAMS[@]}"; do
    PROGRAM_ID=$(grep "^${program}" Anchor.toml | awk -F'"' '{print $2}')
    if [ -n "$PROGRAM_ID" ]; then
        if solana program show "$PROGRAM_ID" > /dev/null 2>&1; then
            echo -e "${GREEN}✅${NC} ${program}: ${PROGRAM_ID}"
        else
            echo -e "${RED}❌${NC} ${program}: Not found on-chain"
        fi
    fi
done

echo ""

