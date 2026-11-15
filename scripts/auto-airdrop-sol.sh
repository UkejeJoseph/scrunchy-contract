#!/bin/bash
# Automated SOL airdrop script with multiple RPC endpoints and local validator support
# This script tries different RPC endpoints and can use local validator to maximize SOL acquisition

set -e

# Check dependencies
if ! command -v bc &> /dev/null; then
    echo "Error: 'bc' command not found. Install it with: sudo apt-get install bc"
    exit 1
fi

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
TARGET_SOL=${1:-7}  # Target SOL amount (default: 7 SOL for deployment)
MAX_ATTEMPTS_PER_ENDPOINT=3
WAIT_BETWEEN_ATTEMPTS=900  # 15 minutes in seconds
WAIT_BETWEEN_ENDPOINTS=60   # 1 minute between endpoint switches
LOCAL_VALIDATOR_PORT=8899

# Track progress
TOTAL_RECEIVED=0
ATTEMPTS=0
SUCCESSFUL_AIRDROPS=0
FAILED_AIRDROPS=0

# RPC Endpoints to try (add your API keys if you have them)
declare -a RPC_ENDPOINTS=(
    "https://api.devnet.solana.com"
    "https://api.devnet.solana.com"  # Try twice with delays
    # Add your custom endpoints here:
    # "https://rpc-devnet.helius.xyz/?api-key=YOUR_KEY"
    # "https://YOUR-ENDPOINT.solana-devnet.quiknode.pro/YOUR_KEY/"
)

# Local validator configuration
USE_LOCAL_VALIDATOR=true
LOCAL_VALIDATOR_URL="http://127.0.0.1:${LOCAL_VALIDATOR_PORT}"

# Function to print colored output
print_status() {
    echo -e "${CYAN}[$(date +'%H:%M:%S')]${NC} $1"
}

print_success() {
    echo -e "${GREEN}✅${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ️${NC} $1"
}

# Function to get current balance
get_balance() {
    local balance_output=$(solana balance 2>/dev/null || echo "0 SOL")
    local balance=$(echo "$balance_output" | awk '{print $1}' | grep -oE '[0-9]+\.?[0-9]*' || echo "0")
    
    # Handle case where balance might be in lamports (very large numbers)
    # If balance > 1000000, it's likely lamports, convert to SOL
    if (( $(echo "$balance > 1000000" | bc -l 2>/dev/null || echo "0") )); then
        balance=$(echo "scale=9; $balance / 1000000000" | bc)
    fi
    
    if [ -z "$balance" ] || [ "$balance" = "" ]; then
        balance="0"
    fi
    echo "$balance"
}

# Function to check if local validator is running
check_local_validator() {
    if curl -s "${LOCAL_VALIDATOR_URL}/health" > /dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Function to start local validator
start_local_validator() {
    print_info "Checking if local validator is running..."
    
    if check_local_validator; then
        print_success "Local validator is already running"
        return 0
    fi
    
    print_info "Starting local validator..."
    
    # Check if solana-test-validator is installed
    if ! command -v solana-test-validator &> /dev/null; then
        print_warning "solana-test-validator not found. Install with: cargo install solana-test-validator"
        return 1
    fi
    
    # Start validator in background
    print_info "Starting validator on port ${LOCAL_VALIDATOR_PORT}..."
    solana-test-validator --reset --quiet > /tmp/solana-validator.log 2>&1 &
    VALIDATOR_PID=$!
    
    # Wait for validator to start
    print_info "Waiting for validator to initialize..."
    sleep 5
    
    # Check if it's running
    local retries=0
    while [ $retries -lt 10 ]; do
        if check_local_validator; then
            print_success "Local validator started successfully (PID: $VALIDATOR_PID)"
            return 0
        fi
        sleep 2
        retries=$((retries + 1))
    done
    
    print_error "Failed to start local validator"
    return 1
}

# Function to airdrop from local validator
airdrop_local() {
    local amount=$1
    
    print_info "Attempting airdrop of ${amount} SOL from local validator..."
    
    # Switch to local validator
    solana config set --url "${LOCAL_VALIDATOR_URL}" > /dev/null 2>&1
    
    # Local validator allows unlimited airdrops
    if solana airdrop "${amount}" > /dev/null 2>&1; then
        sleep 2  # Wait for balance to update
        local new_balance=$(get_balance)
        print_success "Received ${amount} SOL from local validator! New balance: ${new_balance} SOL"
        return 0
    else
        print_error "Failed to airdrop from local validator"
        return 1
    fi
}

# Function to try airdrop from an RPC endpoint
try_airdrop() {
    local endpoint=$1
    local amount=$2
    local attempt=$3
    
    print_status "Attempt ${attempt}/${MAX_ATTEMPTS_PER_ENDPOINT}: Requesting ${amount} SOL from ${endpoint}"
    
    # Switch to endpoint
    solana config set --url "${endpoint}" > /dev/null 2>&1
    
    # Try airdrop
    if solana airdrop "${amount}" > /dev/null 2>&1; then
        sleep 3  # Wait for balance to update
        local new_balance=$(get_balance)
        local received=$(echo "$new_balance - $TOTAL_RECEIVED" | bc)
        print_success "Received ${amount} SOL! New balance: ${new_balance} SOL"
        TOTAL_RECEIVED=$(echo "$TOTAL_RECEIVED + $received" | bc)
        SUCCESSFUL_AIRDROPS=$((SUCCESSFUL_AIRDROPS + 1))
        return 0
    else
        print_warning "Rate limit hit or request failed"
        FAILED_AIRDROPS=$((FAILED_AIRDROPS + 1))
        return 1
    fi
}

# Function to try airdrop with different amounts
try_airdrop_with_fallback() {
    local endpoint=$1
    local attempt=$2
    
    # Try 2 SOL first (maximum usually allowed)
    if try_airdrop "$endpoint" 2 "$attempt"; then
        return 0
    fi
    
    # Fallback to 1 SOL
    print_info "Trying smaller amount (1 SOL)..."
    if try_airdrop "$endpoint" 1 "$attempt"; then
        return 0
    fi
    
    # Fallback to 0.5 SOL
    print_info "Trying even smaller amount (0.5 SOL)..."
    if try_airdrop "$endpoint" 0.5 "$attempt"; then
        return 0
    fi
    
    return 1
}

# Main function
main() {
    echo ""
    echo "🚀 Automated SOL Airdrop Script"
    echo "================================"
    echo ""
    
    # Get initial balance
    local initial_balance=$(get_balance)
    TOTAL_RECEIVED=$initial_balance
    
    print_info "Initial balance: ${initial_balance} SOL"
    print_info "Target balance: ${TARGET_SOL} SOL"
    print_info "Need: $(echo "$TARGET_SOL - $initial_balance" | bc) SOL"
    echo ""
    
    # Check if we already have enough
    if (( $(echo "$initial_balance >= $TARGET_SOL" | bc -l) )); then
        print_success "You already have enough SOL! (${initial_balance} >= ${TARGET_SOL})"
        return 0
    fi
    
    # Try local validator first if enabled (but don't exit - continue to devnet)
    if [ "$USE_LOCAL_VALIDATOR" = true ]; then
        print_info "Attempting to use local validator..."
        if start_local_validator; then
            print_info "Using local validator for unlimited airdrops..."
            
            local needed=$(echo "$TARGET_SOL - $initial_balance" | bc)
            local airdrop_amount=$(echo "$needed + 1" | bc)  # Add buffer
            
            if airdrop_local "$airdrop_amount"; then
                local local_balance=$(get_balance)
                print_success "Successfully obtained SOL from local validator!"
                print_info "Local validator balance: ${local_balance} SOL"
                
                # Switch back to devnet for actual deployment
                print_info "Switching back to devnet for deployment..."
                solana config set --url "https://api.devnet.solana.com" > /dev/null 2>&1
                
                # Get devnet balance
                local devnet_balance=$(get_balance)
                print_info "Devnet balance: ${devnet_balance} SOL"
                
                echo ""
                print_warning "Note: Local validator SOL cannot be used on devnet!"
                print_info "Continuing to collect devnet SOL for deployment..."
                echo ""
                
                # Update initial balance to current devnet balance
                initial_balance=$devnet_balance
                TOTAL_RECEIVED=$devnet_balance
            fi
        else
            print_warning "Local validator not available, continuing with RPC endpoints..."
        fi
        echo ""
    fi
    
    # Try RPC endpoints
    print_info "Trying RPC endpoints..."
    echo ""
    
    local endpoint_index=0
    local consecutive_failures=0
    local max_consecutive_failures=5
    
    while (( $(echo "$TOTAL_RECEIVED < $TARGET_SOL" | bc -l) )); do
        # Check if we've tried too many times
        if [ $consecutive_failures -ge $max_consecutive_failures ]; then
            print_error "Too many consecutive failures. Stopping."
            break
        fi
        
        # Cycle through endpoints
        local endpoint="${RPC_ENDPOINTS[$endpoint_index]}"
        endpoint_index=$((endpoint_index + 1))
        if [ $endpoint_index -ge ${#RPC_ENDPOINTS[@]} ]; then
            endpoint_index=0
            print_info "Cycling back to first endpoint..."
            print_info "Waiting ${WAIT_BETWEEN_ENDPOINTS} seconds before retrying..."
            sleep $WAIT_BETWEEN_ENDPOINTS
        fi
        
        # Try airdrop with this endpoint
        local success=false
        for attempt in $(seq 1 $MAX_ATTEMPTS_PER_ENDPOINT); do
            ATTEMPTS=$((ATTEMPTS + 1))
            
            if try_airdrop_with_fallback "$endpoint" "$attempt"; then
                success=true
                consecutive_failures=0
                break
            fi
            
            # Wait before next attempt
            if [ $attempt -lt $MAX_ATTEMPTS_PER_ENDPOINT ]; then
                print_info "Waiting ${WAIT_BETWEEN_ATTEMPTS} seconds before next attempt..."
                sleep $WAIT_BETWEEN_ATTEMPTS
            fi
        done
        
        if [ "$success" = false ]; then
            consecutive_failures=$((consecutive_failures + 1))
            print_warning "All attempts failed for this endpoint. Trying next endpoint..."
            sleep $WAIT_BETWEEN_ENDPOINTS
        else
            # Check if we've reached target
            local current_balance=$(get_balance)
            if (( $(echo "$current_balance >= $TARGET_SOL" | bc -l) )); then
                print_success "Target reached! Current balance: ${current_balance} SOL"
                break
            fi
        fi
        
        # Small delay between endpoint switches
        sleep 10
    done
    
    # Final summary
    echo ""
    echo "📊 Summary"
    echo "=========="
    local final_balance=$(get_balance)
    print_info "Initial balance: ${initial_balance} SOL"
    print_info "Final balance: ${final_balance} SOL"
    print_info "Total received: $(echo "$final_balance - $initial_balance" | bc) SOL"
    print_info "Successful airdrops: ${SUCCESSFUL_AIRDROPS}"
    print_info "Failed attempts: ${FAILED_AIRDROPS}"
    print_info "Total attempts: ${ATTEMPTS}"
    echo ""
    
    if (( $(echo "$final_balance >= $TARGET_SOL" | bc -l) )); then
        print_success "✅ You have enough SOL to deploy! (${final_balance} >= ${TARGET_SOL})"
    else
        print_warning "⚠️  You still need $(echo "$TARGET_SOL - $final_balance" | bc) more SOL"
        print_info "💡 Tips:"
        print_info "   • Wait 15-30 minutes and run this script again"
        print_info "   • Try using faucet websites: https://faucet.solana.com/"
        print_info "   • Use local validator for testing (run with --local-only)"
        print_info "   • Consider deploying programs one at a time"
    fi
    echo ""
}

# Handle script arguments
case "${2:-}" in
    --local-only)
        USE_LOCAL_VALIDATOR=true
        # Remove RPC endpoints, only use local
        RPC_ENDPOINTS=()
        ;;
    --devnet-only|--no-local)
        USE_LOCAL_VALIDATOR=false
        print_info "Devnet-only mode: Skipping local validator"
        ;;
esac

# Run main function
main

