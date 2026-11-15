# Solana Devnet Airdrop Rules & Limitations

## 🚨 Current Issue

You're hitting devnet airdrop rate limits. Here's what you need to know:

## Devnet Airdrop Rules

### Rate Limits

1. **Request Frequency:**
   - Maximum ~2-3 requests per 10-15 minutes
   - Rate limits are per IP address (not per wallet)
   - Limits reset after a cooldown period

2. **Amount Limits:**
   - Maximum per request: **2 SOL** (sometimes 1 SOL)
   - Maximum per day: ~10-20 SOL (varies)
   - Smaller amounts (0.5 SOL) may still hit rate limits

3. **Cooldown Period:**
   - After hitting rate limit: **Wait 10-15 minutes**
   - Sometimes up to 30 minutes for repeated violations

### Why You're Hitting Limits

Looking at your terminal:

1. ✅ First airdrop: 2 SOL - **Rate limit hit**
2. ✅ Second airdrop: 1 SOL - **Success** (got 2 SOL total)
3. ❌ Third airdrop: 0.5 SOL - **Rate limit hit again**

**You currently have 2 SOL**, but deployment needs **~2.33 SOL** (2.32461912 + 0.00176 fee).

## Solutions

### Solution 1: Wait and Retry (Recommended)

```bash
# Wait 10-15 minutes, then try again
solana airdrop 1

# Or wait 30 minutes for a larger amount
solana airdrop 2
```

### Solution 2: Use Alternative RPC Endpoints

Different RPC providers may have different rate limits:

```bash
# Option A: Use Helius (if you have API key)
solana config set --url https://rpc-devnet.helius.xyz/?api-key=YOUR_KEY

# Option B: Use QuickNode (if you have endpoint)
solana config set --url https://YOUR-ENDPOINT.solana-devnet.quiknode.pro/YOUR_KEY/

# Option C: Use public alternative
solana config set --url https://api.devnet.solana.com
```

### Solution 3: Use Multiple Wallets

Create additional wallets to distribute airdrop requests:

```bash
# Create new wallet
solana-keygen new --outfile ~/.config/solana/wallet2.json

# Switch to new wallet
solana config set --keypair ~/.config/solana/wallet2.json

# Airdrop to new wallet
solana airdrop 2

# Transfer to main wallet
solana transfer <main-wallet-pubkey> 1.5

# Switch back
solana config set --keypair ~/.config/solana/id.json
```

### Solution 4: Request Smaller Amounts Over Time

```bash
# Request 1 SOL, wait 10 minutes
solana airdrop 1
# Wait 10-15 minutes...

# Request another 1 SOL
solana airdrop 1
```

### Solution 5: Use Faucet Websites

Some websites provide devnet SOL without rate limits:

1. **Solana Faucet:**
   - <https://faucet.solana.com/>
   - Enter your wallet address
   - Usually gives 1-2 SOL

2. **QuickNode Faucet:**
   - <https://faucet.quicknode.com/solana/devnet>
   - Requires QuickNode account (free)

3. **Helius Faucet:**
   - <https://helius.dev/faucet>
   - Requires Helius account (free)

## Best Practices

### 1. Request Only What You Need

```bash
# Check current balance
solana balance

# Request only what's needed
# You need ~2.33 SOL, you have 2 SOL
# Request 1 SOL (not 2)
solana airdrop 1
```

### 2. Space Out Requests

```bash
# Don't spam airdrop requests
# Wait at least 10-15 minutes between requests
```

### 3. Use a Script with Delays

```bash
#!/bin/bash
# Wait and retry airdrop

MAX_ATTEMPTS=5
WAIT_TIME=900  # 15 minutes in seconds

for i in $(seq 1 $MAX_ATTEMPTS); do
    echo "Attempt $i/$MAX_ATTEMPTS"
    if solana airdrop 1; then
        echo "✅ Airdrop successful!"
        break
    else
        if [ $i -lt $MAX_ATTEMPTS ]; then
            echo "⏳ Waiting $WAIT_TIME seconds before retry..."
            sleep $WAIT_TIME
        fi
    fi
done
```

## Current Status

**Your Situation:**

- Current balance: **2 SOL**
- Needed for deployment: **~2.33 SOL**
- Shortfall: **~0.33 SOL**

**Next Steps:**

1. Wait 10-15 minutes
2. Request 1 SOL: `solana airdrop 1`
3. Should give you ~3 SOL total (enough for deployment)

## Alternative: Deploy Programs Individually

If you can't get more SOL immediately, you can deploy programs one at a time (each costs less):

```bash
# Deploy smallest program first
anchor deploy --program-name access_control

# Wait, then deploy next
anchor deploy --program-name asset_registry

# Finally deploy marketplace
anchor deploy --program-name marketplace
```

However, each deployment still requires rent for the program account (~2.3 SOL per program).

## Understanding Devnet Costs

**Per Program Deployment:**

- Program account rent: ~2.3 SOL
- Transaction fees: ~0.00176 SOL
- **Total per program: ~2.33 SOL**

**For 3 programs:**

- Total needed: ~7 SOL
- You have: 2 SOL
- Need: ~5 more SOL

## Recommended Approach

1. **Wait 15 minutes** (let rate limit reset)
2. **Request 2 SOL**: `solana airdrop 2`
3. **Wait another 15 minutes** (if needed)
4. **Request 2 more SOL**: `solana airdrop 2`
5. **Deploy all programs**: `anchor deploy`

Or use a faucet website to get SOL without rate limits.

## Rate Limit Bypass Tips

1. **Use VPN** (different IP = different rate limit)
2. **Use different RPC endpoint** (some have higher limits)
3. **Use faucet websites** (no rate limits)
4. **Wait patiently** (limits reset automatically)

## Summary

**Devnet Airdrop Rules:**

- ✅ Max 2-3 requests per 10-15 minutes
- ✅ Max 2 SOL per request
- ✅ Rate limits are per IP address
- ✅ Cooldown: 10-30 minutes

**Your Options:**

1. ⏳ Wait 15 minutes, then `solana airdrop 1`
2. 🌐 Use faucet website (no rate limits)
3. 🔄 Use alternative RPC endpoint
4. 💼 Use multiple wallets

**Recommended:** Wait 15 minutes, then request 1-2 SOL. You should have enough for deployment!
