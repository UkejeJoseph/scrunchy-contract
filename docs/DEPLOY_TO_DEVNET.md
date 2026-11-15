# Deploy to Solana Devnet

## ✅ Prerequisites Check

Your setup looks good:
- ✅ Solana CLI configured for devnet
- ✅ Wallet has 1 SOL (sufficient for deployment)
- ✅ Programs compiled (`.so` files ready)
- ✅ Anchor.toml configured with devnet program IDs

## Step-by-Step Deployment

### Step 1: Verify Configuration

```bash
cd scrunchy-contract

# Check Solana config
solana config get

# Should show:
# RPC URL: https://api.devnet.solana.com
# Keypair Path: ~/.config/solana/id.json
```

### Step 2: Check Wallet Balance

```bash
# Check your wallet balance
solana balance

# If you need more SOL (for devnet only):
solana airdrop 2
```

**Note**: Devnet SOL is free - you can airdrop as needed.

### Step 3: Build Programs (if not already done)

```bash
# Build all programs
anchor build

# Verify .so files exist
ls -lh target/deploy/*.so
```

You should see:
- `access_control.so` (~298K)
- `asset_registry.so` (~325K)
- `marketplace.so` (~326K)

### Step 4: Deploy Programs

Anchor will deploy all programs defined in `Anchor.toml`:

```bash
# Deploy all programs to devnet
anchor deploy
```

This will:
1. Deploy `access_control` program
2. Deploy `asset_registry` program
3. Deploy `marketplace` program
4. Update program IDs in your code if needed

**Expected Output:**
```
Deploying cluster: https://api.devnet.solana.com
Upgrade authority: <your-wallet-pubkey>
Deploying program "access_control"...
Program Id: n41USxoi9Lf1RyZUVAfCHD5TSeuDBuqDxEpUHzkDqf2
Deploying program "asset_registry"...
Program Id: 7vxpftgcdbh6NW77uAGVxp4aM2RcbQtmeGixNp2ifCDn
Deploying program "marketplace"...
Program Id: G46EH6LskmJpBTvZNewSL4Xe5fkdbcdkMpRM8wQ9T3RH
```

### Step 5: Verify Deployment

```bash
# Check program accounts
solana program show n41USxoi9Lf1RyZUVAfCHD5TSeuDBuqDxEpUHzkDqf2
solana program show 7vxpftgcdbh6NW77uAGVxp4aM2RcbQtmeGixNp2ifCDn
solana program show G46EH6LskmJpBTvZNewSL4Xe5fkdbcdkMpRM8wQ9T3RH
```

You should see:
- Program Data Account
- Program ID
- Authority
- Last Deployed Slot

### Step 6: Update Program IDs (if needed)

If Anchor generated new program IDs during deployment, it will automatically update:
- `Anchor.toml` - `[programs.devnet]` section
- Rust files - `declare_id!()` macros
- TypeScript/IDL files

**Verify the IDs match:**
```bash
# Check Anchor.toml
grep -A 3 "\[programs.devnet\]" Anchor.toml

# Check Rust files
grep "declare_id!" programs/*/src/lib.rs
```

### Step 7: Rebuild and Verify

After deployment, rebuild to ensure everything is in sync:

```bash
# Rebuild to sync IDs
anchor build

# Verify IDL files are updated
ls -lh target/idl/*.json
```

## Alternative: Deploy Individual Programs

If you need to deploy programs one at a time:

```bash
# Deploy specific program
anchor deploy --program-name access_control
anchor deploy --program-name asset_registry
anchor deploy --program-name marketplace
```

## Troubleshooting

### Error: Insufficient Funds

```bash
# Airdrop more SOL (devnet only)
solana airdrop 2

# Check balance
solana balance
```

### Error: Program Already Deployed

If you get "program already deployed" error:

```bash
# Option 1: Upgrade existing program
anchor upgrade target/deploy/access_control.so --program-id n41USxoi9Lf1RyZUVAfCHD5TSeuDBuqDxEpUHzkDqf2

# Option 2: Close and redeploy (WARNING: This closes the program!)
solana program close <program-id> --bypass-warning
anchor deploy
```

### Error: Program ID Mismatch

If program IDs don't match:

```bash
# Sync program IDs
anchor keys sync

# Then rebuild
anchor build
```

### Error: Network Issues

If you encounter network timeouts:

```bash
# Use a different RPC endpoint (Helius, QuickNode, etc.)
# Update Anchor.toml [provider] section or use environment variable:
export ANCHOR_PROVIDER_URL=https://api.devnet.solana.com
anchor deploy
```

## Verify Deployment on Explorer

After deployment, verify on Solana Explorer:

1. **Access Control:**
   https://explorer.solana.com/address/n41USxoi9Lf1RyZUVAfCHD5TSeuDBuqDxEpUHzkDqf2?cluster=devnet

2. **Asset Registry:**
   https://explorer.solana.com/address/7vxpftgcdbh6NW77uAGVxp4aM2RcbQtmeGixNp2ifCDn?cluster=devnet

3. **Marketplace:**
   https://explorer.solana.com/address/G46EH6LskmJpBTvZNewSL4Xe5fkdbcdkMpRM8wQ9T3RH?cluster=devnet

## Post-Deployment Checklist

- [ ] All three programs deployed successfully
- [ ] Program IDs match in `Anchor.toml` and Rust files
- [ ] IDL files generated and up-to-date
- [ ] Programs visible on Solana Explorer
- [ ] Wallet has sufficient SOL for transactions
- [ ] Backend can connect to deployed programs

## Update Backend Configuration

After deployment, update your backend with the deployed program IDs:

```typescript
// scrunchy-backend/src/lib/programs.ts
export const PROGRAM_IDS = {
  assetRegistry: new PublicKey("7vxpftgcdbh6NW77uAGVxp4aM2RcbQtmeGixNp2ifCDn"),
  marketplace: new PublicKey("G46EH6LskmJpBTvZNewSL4Xe5fkdbcdkMpRM8wQ9T3RH"),
  accessControl: new PublicKey("n41USxoi9Lf1RyZUVAfCHD5TSeuDBuqDxEpUHzkDqf2"),
};
```

## Next Steps

1. ✅ Deploy to devnet
2. ⏭️ Test with devnet transactions
3. ⏭️ Update backend to use deployed programs
4. ⏭️ Test full integration
5. ⏭️ Deploy to mainnet (when ready)

## Quick Deploy Command

For a quick deployment, run:

```bash
cd scrunchy-contract
anchor build && anchor deploy
```

This builds and deploys all programs in one command.

## Cost Estimate

**Devnet (Free):**
- Deployment: Free (devnet SOL is free)
- Transactions: Free

**Mainnet (Real SOL):**
- Program deployment: ~2-5 SOL per program
- Account creation: ~0.001 SOL per account
- Transactions: ~0.000005 SOL per transaction

For devnet, you can airdrop as much SOL as needed:
```bash
solana airdrop 10  # Get 10 SOL on devnet
```

