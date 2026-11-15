# Quick Deploy Reference

## 🚀 Deploy to Devnet (One Command)

```bash
cd scrunchy-contract
anchor deploy
```

## 📋 Pre-Deployment Checklist

- [x] Solana CLI configured for devnet
- [x] Wallet has SOL (1+ SOL recommended)
- [x] Programs compiled (`.so` files exist)
- [x] `Anchor.toml` has devnet program IDs

## 🔧 Common Commands

```bash
# Check configuration
solana config get

# Check balance
solana balance

# Airdrop SOL (devnet only)
solana airdrop 2

# Build programs
anchor build

# Deploy all programs
anchor deploy

# Deploy specific program
anchor deploy --program-name access_control

# Verify program
solana program show <program-id>

# Check program on explorer
# https://explorer.solana.com/address/<program-id>?cluster=devnet
```

## 📍 Your Program IDs (Devnet)

- **Access Control:** `n41USxoi9Lf1RyZUVAfCHD5TSeuDBuqDxEpUHzkDqf2`
- **Asset Registry:** `7vxpftgcdbh6NW77uAGVxp4aM2RcbQtmeGixNp2ifCDn`
- **Marketplace:** `G46EH6LskmJpBTvZNewSL4Xe5fkdbcdkMpRM8wQ9T3RH`

## 🐛 Troubleshooting

**Insufficient funds:**
```bash
solana airdrop 2
```

**Program already deployed:**
```bash
anchor upgrade target/deploy/<program>.so --program-id <program-id>
```

**Network issues:**
```bash
export ANCHOR_PROVIDER_URL=https://api.devnet.solana.com
anchor deploy
```

## ✅ After Deployment

1. Verify programs on Solana Explorer
2. Update backend with program IDs
3. Test with devnet transactions
4. Rebuild to sync IDs: `anchor build`

