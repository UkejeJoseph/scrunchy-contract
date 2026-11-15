# Deployment Cost Estimate

## 📊 Total Deployment Costs

**For all 3 programs: ~6.98 SOL**

### Program-by-Program Breakdown

| Program | Size | Rent Exemption | Transaction Fee | **Total Cost** |
|---------|------|----------------|-----------------|----------------|
| `access_control` | 297 KB | ~2.19 SOL | ~0.00176 SOL | **~2.19 SOL** |
| `asset_registry` | 324 KB | ~2.39 SOL | ~0.00176 SOL | **~2.39 SOL** |
| `marketplace` | 326 KB | ~2.40 SOL | ~0.00176 SOL | **~2.40 SOL** |
| **TOTAL** | **947 KB** | **~6.98 SOL** | **~0.005 SOL** | **~6.98 SOL** |

## 💰 Current Status

- **Current Balance:** 2 SOL
- **Required:** ~6.98 SOL
- **Shortfall:** ~5.0 SOL

## 🚀 Quick Calculation Script

Run this to get the latest costs:

```bash
cd scrunchy-contract
bash scripts/calculate-deployment-costs.sh
```

## 📝 Cost Details

### Per Program Costs

Each program deployment requires:

1. **Rent Exemption** (~2.19-2.40 SOL)
   - Based on program size (297-326 KB)
   - One-time payment to make account rent-exempt
   - Varies slightly by program size

2. **Transaction Fee** (~0.00176 SOL)
   - Network fee for the deployment transaction
   - Standard Solana transaction fee

### Total for All Programs

- **3 programs × ~2.33 SOL average = ~6.98 SOL**
- Plus small buffer for safety: **~7 SOL recommended**

## 💡 Getting More SOL on Devnet

You need approximately **5 more SOL** to deploy all programs.

### Option 1: Airdrop (with rate limits)
```bash
# Wait 10-15 minutes between requests
solana airdrop 2  # Request 2 SOL
# Wait 15 minutes...
solana airdrop 2  # Request another 2 SOL
# Wait 15 minutes...
solana airdrop 1  # Request final 1 SOL
```

### Option 2: Use Faucet Websites (no rate limits)
- https://faucet.solana.com/
- https://faucet.quicknode.com/solana/devnet
- https://helius.dev/faucet

### Option 3: Deploy One at a Time
```bash
# Deploy smallest first (access_control: ~2.19 SOL)
anchor deploy --program-name access_control

# Wait, get more SOL, then deploy next
anchor deploy --program-name asset_registry

# Finally deploy marketplace
anchor deploy --program-name marketplace
```

## ✅ Deployment Command

Once you have sufficient balance:

```bash
cd scrunchy-contract
anchor deploy
```

This will deploy all 3 programs in sequence.

## 📌 Notes

- Costs are approximate and may vary slightly based on network conditions
- Rent exemption is calculated based on program size
- Transaction fees are minimal (~0.00176 SOL per program)
- All costs are in **devnet SOL** (free, but rate-limited)
- For mainnet, costs would be similar but using real SOL

