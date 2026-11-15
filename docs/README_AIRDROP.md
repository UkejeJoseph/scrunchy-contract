# Quick Reference: Automated SOL Airdrop

## 🚀 Quick Commands

### Get SOL for Deployment (7 SOL target)
```bash
bash scripts/auto-airdrop-sol.sh
```

### Get Custom Amount
```bash
bash scripts/auto-airdrop-sol.sh 10  # Get 10 SOL
```

### Use Local Validator Only (for testing)
```bash
bash scripts/start-local-validator.sh
bash scripts/auto-airdrop-sol.sh 100 --local-only
```

## 📋 What the Script Does

1. ✅ Checks your current balance
2. ✅ Tries local validator first (if available)
3. ✅ Switches between RPC endpoints
4. ✅ Handles rate limits automatically
5. ✅ Tries different amounts (2 SOL → 1 SOL → 0.5 SOL)
6. ✅ Shows progress and statistics

## 🎯 Common Use Cases

### Case 1: Need SOL for Devnet Deployment
```bash
# Calculate how much you need
bash scripts/calculate-deployment-costs.sh

# Get the SOL automatically
bash scripts/auto-airdrop-sol.sh 7

# Deploy when ready
anchor deploy
```

### Case 2: Local Testing (Unlimited SOL)
```bash
# Start local validator
bash scripts/start-local-validator.sh

# Switch to local
solana config set --url http://127.0.0.1:8899

# Get unlimited SOL
solana airdrop 1000

# Test your programs
anchor test
```

### Case 3: Background Collection
```bash
# Run in background
nohup bash scripts/auto-airdrop-sol.sh 10 > airdrop.log 2>&1 &

# Check progress
tail -f airdrop.log
```

## ⚙️ Configuration

Edit `scripts/auto-airdrop-sol.sh` to:
- Add custom RPC endpoints (with API keys)
- Adjust wait times
- Change retry logic

## 📊 Script Output

The script shows:
- Current balance
- Target balance
- Progress updates
- Success/failure counts
- Final summary

## 🔧 Troubleshooting

**Script fails immediately?**
- Check: `solana --version`
- Check: `bc --version` (install with `sudo apt-get install bc`)

**Rate limits still hit?**
- Increase wait times in script
- Add more RPC endpoints
- Use local validator for testing

**Local validator won't start?**
- Install: `cargo install solana-test-validator`
- Check port 8899 is free: `lsof -i :8899`

