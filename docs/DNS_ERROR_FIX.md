# DNS Error Fix Guide

## 🚨 Error: DNS Lookup Failure

```
Error: dns error: failed to lookup address information: No address associated with hostname
```

This means your system cannot resolve `api.devnet.solana.com` to an IP address.

## Common Causes

### 1. **WSL2 DNS Issues** (Most Likely - You're on WSL2)

WSL2 sometimes has DNS resolution problems, especially after Windows sleep/wake or network changes.

### 2. **Network Connectivity**
- Internet connection down
- Firewall blocking DNS
- VPN interfering with DNS

### 3. **DNS Server Issues**
- DNS server not responding
- Incorrect DNS configuration

### 4. **Temporary Outage**
- Solana RPC endpoint temporarily down
- DNS propagation delay

## Solutions

### Solution 1: Fix WSL2 DNS (Recommended)

WSL2 DNS issues are common. Fix them:

```bash
# Option A: Restart WSL2 DNS service
sudo service systemd-resolved restart

# Option B: Update DNS servers in WSL2
sudo bash -c 'echo "nameserver 8.8.8.8" > /etc/resolv.conf'
sudo bash -c 'echo "nameserver 8.8.4.4" >> /etc/resolv.conf'

# Option C: Use Windows DNS (get from Windows)
# In PowerShell (Windows): ipconfig /all | findstr "DNS"
# Then add those DNS servers to /etc/resolv.conf
```

**Permanent Fix for WSL2:**

Create `/etc/wsl.conf`:
```bash
sudo tee /etc/wsl.conf > /dev/null <<EOF
[network]
generateResolvConf = false
EOF
```

Then update `/etc/resolv.conf`:
```bash
sudo rm /etc/resolv.conf
sudo bash -c 'cat > /etc/resolv.conf <<EOF
nameserver 8.8.8.8
nameserver 8.8.4.4
EOF'
```

**Restart WSL2:**
```powershell
# In Windows PowerShell (as admin):
wsl --shutdown
# Then restart your WSL2 terminal
```

### Solution 2: Use IP Address Directly

Temporarily bypass DNS by using IP address:

```bash
# Find IP address
nslookup api.devnet.solana.com 8.8.8.8

# Or use alternative RPC endpoint
solana config set --url https://api.devnet.solana.com
```

### Solution 3: Use Alternative RPC Endpoint

Use a different Solana RPC provider:

```bash
# Option A: Use public alternative
solana config set --url https://api.devnet.solana.com

# Option B: Use Helius (if you have API key)
solana config set --url https://rpc-devnet.helius.xyz/?api-key=YOUR_KEY

# Option C: Use QuickNode (if you have endpoint)
solana config set --url https://YOUR-ENDPOINT.solana-devnet.quiknode.pro/YOUR_KEY/

# Option D: Use GenesysGo
solana config set --url https://devnet.genesysgo.net/
```

### Solution 4: Check Network Connectivity

```bash
# Test internet connectivity
ping -c 3 8.8.8.8

# Test DNS resolution
nslookup google.com

# Test HTTPS connectivity
curl -I https://www.google.com
```

### Solution 5: Restart Network Services

```bash
# Restart networking (Ubuntu/Debian)
sudo systemctl restart networking

# Or restart network manager
sudo systemctl restart NetworkManager

# Or restart systemd-resolved
sudo systemctl restart systemd-resolved
```

### Solution 6: Use Windows Hosts File (WSL2)

If DNS continues to fail, add to Windows hosts file:

1. **Find IP address:**
   ```bash
   # In WSL2, try:
   nslookup api.devnet.solana.com 8.8.8.8
   ```

2. **Edit Windows hosts file:**
   ```powershell
   # In Windows PowerShell (as admin):
   notepad C:\Windows\System32\drivers\etc\hosts
   ```

3. **Add entry:**
   ```
   <IP_ADDRESS> api.devnet.solana.com
   ```

## Quick Diagnostic Commands

Run these to diagnose:

```bash
# 1. Check DNS resolution
nslookup api.devnet.solana.com

# 2. Check connectivity
ping -c 3 api.devnet.solana.com

# 3. Check HTTPS
curl -I https://api.devnet.solana.com

# 4. Check current config
solana config get

# 5. Test with different DNS
nslookup api.devnet.solana.com 8.8.8.8
```

## Most Likely Fix (WSL2)

Since you're on WSL2, try this first:

```bash
# 1. Update DNS servers
sudo bash -c 'cat > /etc/resolv.conf <<EOF
nameserver 8.8.8.8
nameserver 8.8.4.4
EOF'

# 2. Test DNS
nslookup api.devnet.solana.com

# 3. If still failing, restart WSL2
# In Windows PowerShell: wsl --shutdown
# Then reopen your terminal

# 4. Try deployment again
anchor deploy
```

## Alternative: Use Different RPC

If DNS continues to fail, use an alternative endpoint:

```bash
# Use GenesysGo (usually more reliable)
solana config set --url https://devnet.genesysgo.net/

# Verify
solana config get

# Try deployment
anchor deploy
```

## Prevention

To prevent future DNS issues in WSL2:

1. **Create `/etc/wsl.conf`:**
   ```bash
   sudo tee /etc/wsl.conf > /dev/null <<EOF
   [network]
   generateResolvConf = false
   EOF
   ```

2. **Set static DNS:**
   ```bash
   sudo bash -c 'cat > /etc/resolv.conf <<EOF
   nameserver 8.8.8.8
   nameserver 8.8.4.4
   nameserver 1.1.1.1
   EOF'
   ```

3. **Make it immutable:**
   ```bash
   sudo chattr +i /etc/resolv.conf
   ```

## Summary

**Most Likely Cause:** WSL2 DNS resolution issue

**Quick Fix:**
```bash
sudo bash -c 'echo "nameserver 8.8.8.8" > /etc/resolv.conf'
sudo bash -c 'echo "nameserver 8.8.4.4" >> /etc/resolv.conf'
nslookup api.devnet.solana.com
anchor deploy
```

**If Still Failing:**
- Use alternative RPC: `solana config set --url https://devnet.genesysgo.net/`
- Restart WSL2: `wsl --shutdown` (in Windows PowerShell)


