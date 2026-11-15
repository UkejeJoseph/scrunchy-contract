# Dependency Conflict Fix

## Issue
Anchor 0.32.1 has a dependency conflict between:
- `anchor-lang` requires `solana-instruction = "^2"` (resolves to 2.3.3)
- `anchor-spl` → `spl-token-2022` → `solana-zk-sdk` requires `solana-instruction = "2.2.1"`

## Solutions

### Option 1: Use Anchor 0.31.0 (Recommended)
```bash
# Update Anchor.toml
anchor_version = "0.31.0"

# Update all Cargo.toml files
anchor-lang = "0.31.0"
anchor-spl = "0.31.0"

# Install Anchor 0.31.0
avm install 0.31.0 --force
avm use 0.31.0
```

### Option 2: Use Anchor 0.30.0
```bash
anchor_version = "0.30.0"
anchor-lang = "0.30.0"
anchor-spl = "0.30.0"
```

### Option 3: Wait for Fix
This is a known issue that should be resolved in future Anchor versions.

### Option 4: Patch Dependencies (Advanced)
Add to workspace `Cargo.toml`:
```toml
[patch.crates-io]
# This may break anchor-lang, use with caution
solana-instruction = "=2.2.1"
```

**Warning**: This may cause other issues as anchor-lang expects 2.3.3.

## Recommended Action
Use Anchor 0.31.0 which is stable and doesn't have this conflict.

