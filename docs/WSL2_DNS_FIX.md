# WSL2 DNS Fix for Anchor Deploy

## 🚨 Problem

**Symptoms:**
- `ping` works ✅
- `curl` works ✅  
- `anchor deploy` fails with DNS error ❌

**Root Cause:**
Anchor uses Rust's DNS resolver, which doesn't work well with WSL2's automatic DNS proxy (`10.255.255.254`). This is a known WSL2 issue.

## ✅ Solution (Applied)

I've fixed your DNS configuration:

1. **Updated `/etc/resolv.conf`** to use public DNS servers:
   - 8.8.8.8 (Google)
   - 8.8.4.4 (Google)
   - 1.1.1.1 (Cloudflare)

2. **Created `/etc/wsl.conf`** to prevent WSL2 from auto-overwriting DNS settings

## 🔄 If You Need to Reapply

If WSL2 resets your DNS (after restart), run:

```bash
# Fix DNS
sudo bash -c 'cat > /etc/resolv.conf <<EOF
nameserver 8.8.8.8
nameserver 8.8.4.4
nameserver 1.1.1.1
EOF'

# Prevent auto-generation
sudo bash -c 'cat > /etc/wsl.conf <<EOF
[network]
generateResolvConf = false
EOF'
```

## 🧪 Test

```bash
# Test DNS resolution
nslookup api.devnet.solana.com

# Test deployment
anchor deploy
```

## 🔒 Make DNS Immutable (Optional)

To prevent accidental changes:

```bash
sudo chattr +i /etc/resolv.conf
```

To remove immutability later:
```bash
sudo chattr -i /etc/resolv.conf
```

## 📝 Why This Happens

WSL2 uses Windows' DNS proxy (`10.255.255.254`) which:
- Works for basic tools (`ping`, `curl`)
- Fails for Rust-based tools (Anchor, Solana CLI)
- Can be unreliable after Windows sleep/wake

Using public DNS directly bypasses this issue.

## ✅ Status

DNS should now work for `anchor deploy`. Try deploying again!


