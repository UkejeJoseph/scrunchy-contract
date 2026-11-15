# Fix: "216 write transactions failed" Error

## 🚨 Error Description

When deploying Solana programs, you may encounter:
```
Error: 216 write transactions failed
```

This error means Anchor tried to deploy your program in chunks (write transactions), but all of them failed.

## 🔍 Common Causes

1. **RPC Endpoint Issues**
   - Public devnet RPC is rate-limited or congested
   - Network connectivity problems
   - RPC endpoint temporarily unavailable

2. **Network Congestion**
   - Devnet experiencing high traffic
   - Too many concurrent deployments

3. **Transaction Size Limits**
   - Program too large for single transaction
   - RPC endpoint can't handle large deployments

## ✅ Solutions

### Solution 1: Deploy Programs One at a Time (Recommended)

Use the individual deployment script:

```bash
cd scrunchy-contract
bash scripts/deploy-individual.sh
```

This will:
- Deploy each program separately
- Retry on failure
- Show detailed progress
- Verify each deployment

### Solution 2: Use Alternative RPC Endpoint

Try a different RPC endpoint with better reliability:

```bash
# Option 1: Use Helius (if you have API key)
solana config set --url https://rpc-devnet.helius.xyz/?api-key=YOUR_KEY

# Option 2: Use QuickNode (if you have endpoint)
solana config set --url https://YOUR-ENDPOINT.solana-devnet.quiknode.pro/YOUR_KEY/

# Option 3: Try public alternative
solana config set --url https://api.devnet.solana.com

# Then deploy
anchor deploy
```

### Solution 3: Deploy Manually One at a Time

```bash
# Deploy smallest first
anchor deploy --program-name access_control

# Wait a moment, then deploy next
anchor deploy --program-name asset_registry

# Finally deploy marketplace
anchor deploy --program-name marketplace
```

### Solution 4: Increase Retry Logic

Add retry logic to your deployment:

```bash
# Try deploying with retries
for i in {1..3}; do
    echo "Attempt $i..."
    if anchor deploy; then
        echo "Success!"
        break
    else
        echo "Failed, waiting 30 seconds..."
        sleep 30
    fi
done
```

### Solution 5: Check Network and Retry

```bash
# Check RPC health
curl -X POST https://api.devnet.solana.com \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'

# If healthy, try deploying again
anchor deploy
```

## 🔧 Troubleshooting Steps

### Step 1: Verify Your Setup

```bash
# Check balance
solana balance

# Check config
solana config get

# Verify programs are built
ls -lh target/deploy/*.so
```

### Step 2: Check RPC Endpoint

```bash
# Test RPC connectivity
curl -X POST https://api.devnet.solana.com \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'

# Should return: {"jsonrpc":"2.0","result":"ok","id":1}
```

### Step 3: Try Different Approach

If all else fails:

1. **Wait and Retry**: Sometimes devnet is just busy
   ```bash
   # Wait 5-10 minutes, then retry
   anchor deploy
   ```

2. **Use Local Validator**: For testing only
   ```bash
   bash scripts/start-local-validator.sh
   solana config set --url http://127.0.0.1:8899
   anchor deploy
   ```

3. **Deploy During Off-Peak Hours**: Less network congestion

## 📊 Program Sizes

Your programs are reasonable sizes:
- `access_control`: 298 KB ✅
- `asset_registry`: 325 KB ✅
- `marketplace`: 326 KB ✅

These should deploy fine - the issue is likely RPC-related, not program size.

## 💡 Best Practices

1. **Deploy One at a Time**: More reliable than deploying all at once
2. **Use Reliable RPC**: Consider paid RPC endpoints for production
3. **Monitor Network**: Check Solana status pages
4. **Have Patience**: Devnet can be unreliable - retry after delays

## 🚀 Quick Fix Command

```bash
# Quick fix: Deploy individually with retries
cd scrunchy-contract
bash scripts/deploy-individual.sh
```

This script handles retries, error checking, and verification automatically.

## 📝 Additional Notes

- **216 transactions** = Anchor splits large programs into chunks
- **All failed** = RPC endpoint issue, not your code
- **Retry works** = Usually temporary network issue
- **Persistent failure** = Try different RPC endpoint

