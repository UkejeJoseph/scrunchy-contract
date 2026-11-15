# Automated SOL Airdrop Guide

## 🚀 Quick Start

### Basic Usage

```bash
cd scrunchy-contract

# Get SOL automatically (targets 7 SOL by default)
bash scripts/auto-airdrop-sol.sh

# Specify custom target amount
bash scripts/auto-airdrop-sol.sh 10  # Get 10 SOL
```

## 📋 Features

- ✅ **Multiple RPC Endpoints**: Automatically switches between different endpoints
- ✅ **Local Validator Support**: Can use local validator for unlimited airdrops
- ✅ **Smart Retry Logic**: Handles rate limits with exponential backoff
- ✅ **Progress Tracking**: Shows real-time progress and statistics
- ✅ **Flexible Amounts**: Tries different amounts (2 SOL, 1 SOL, 0.5 SOL)

## 🎯 Usage Modes

### 1. Standard Mode (RPC Endpoints)

Tries multiple RPC endpoints with rate limit handling:

```bash
bash scripts/auto-airdrop-sol.sh 7
```

This will:
- Try different RPC endpoints
- Handle rate limits automatically
- Wait 15 minutes between attempts
- Cycle through endpoints

### 2. Local Validator Mode

Use local validator for unlimited airdrops (great for testing):

```bash
# Start local validator first
bash scripts/start-local-validator.sh

# Then use it
bash scripts/auto-airdrop-sol.sh 100 --local-only
```

**Note**: Local validator SOL cannot be used on devnet! It's only for local testing.

### 3. Hybrid Mode (Default)

Tries local validator first, then falls back to RPC endpoints:

```bash
bash scripts/auto-airdrop-sol.sh 7
```

## 🔧 Configuration

### Adding Custom RPC Endpoints

Edit `scripts/auto-airdrop-sol.sh` and add your endpoints:

```bash
declare -a RPC_ENDPOINTS=(
    "https://api.devnet.solana.com"
    "https://rpc-devnet.helius.xyz/?api-key=YOUR_KEY"
    "https://YOUR-ENDPOINT.solana-devnet.quiknode.pro/YOUR_KEY/"
)
```

### Adjusting Wait Times

Edit these variables in the script:

```bash
WAIT_BETWEEN_ATTEMPTS=900   # 15 minutes between attempts
WAIT_BETWEEN_ENDPOINTS=60   # 1 minute between endpoint switches
MAX_ATTEMPTS_PER_ENDPOINT=3 # Max attempts per endpoint
```

## 📊 Example Output

```
🚀 Automated SOL Airdrop Script
================================

ℹ️ Initial balance: 2 SOL
ℹ️ Target balance: 7 SOL
ℹ️ Need: 5 SOL

[10:30:15] Attempting to use local validator...
[10:30:20] ✅ Local validator started successfully (PID: 12345)
[10:30:25] ✅ Received 5 SOL from local validator! New balance: 7 SOL

📊 Summary
==========
ℹ️ Initial balance: 2 SOL
ℹ️ Final balance: 7 SOL
ℹ️ Total received: 5 SOL
ℹ️ Successful airdrops: 1
ℹ️ Failed attempts: 0
ℹ️ Total attempts: 1

✅ You have enough SOL to deploy! (7 >= 7)
```

## 🛠️ Local Validator Setup

### Start Local Validator

```bash
bash scripts/start-local-validator.sh
```

This will:
- Start a local Solana validator on port 8899
- Allow unlimited airdrops
- Perfect for testing and development

### Switch to Local Validator

```bash
solana config set --url http://127.0.0.1:8899
solana airdrop 1000  # Get 1000 SOL instantly!
```

### Switch Back to Devnet

```bash
solana config set --url https://api.devnet.solana.com
```

## 💡 Tips & Tricks

### 1. Run in Background

```bash
nohup bash scripts/auto-airdrop-sol.sh 10 > airdrop.log 2>&1 &
```

### 2. Monitor Progress

```bash
tail -f airdrop.log
```

### 3. Stop Script

```bash
pkill -f auto-airdrop-sol.sh
```

### 4. Use with Deployment Script

```bash
# Get SOL first
bash scripts/auto-airdrop-sol.sh 7

# Then deploy
anchor deploy
```

## ⚠️ Important Notes

1. **Rate Limits**: Devnet has strict rate limits (2-3 requests per 15 minutes)
2. **Local Validator**: SOL from local validator cannot be used on devnet
3. **Patience**: The script will wait between attempts to respect rate limits
4. **API Keys**: Add your RPC provider API keys for better rate limits

## 🔄 Workflow Examples

### For Development (Local Testing)

```bash
# 1. Start local validator
bash scripts/start-local-validator.sh

# 2. Switch to local
solana config set --url http://127.0.0.1:8899

# 3. Get unlimited SOL
solana airdrop 1000

# 4. Build and test
anchor build
anchor test
```

### For Devnet Deployment

```bash
# 1. Switch to devnet
solana config set --url https://api.devnet.solana.com

# 2. Get SOL (with rate limit handling)
bash scripts/auto-airdrop-sol.sh 7

# 3. Deploy
anchor deploy
```

## 🐛 Troubleshooting

### Script Fails Immediately

- Check your internet connection
- Verify Solana CLI is installed: `solana --version`
- Check RPC endpoint is accessible: `curl https://api.devnet.solana.com/health`

### Local Validator Won't Start

- Install: `cargo install solana-test-validator`
- Check port 8899 is available: `lsof -i :8899`
- Check logs: `tail -f /tmp/solana-validator.log`

### Rate Limits Still Hit

- Increase `WAIT_BETWEEN_ATTEMPTS` in the script
- Add more RPC endpoints with API keys
- Use local validator for testing instead

## 📚 Related Scripts

- `scripts/calculate-deployment-costs.sh` - Calculate exact deployment costs
- `scripts/deploy-devnet.sh` - Deploy to devnet
- `scripts/start-local-validator.sh` - Start local validator

