# Fix IDL Generation Issue

## Problem

IDL generation fails with "Error: IDL doesn't exist" due to Anchor CLI version mismatch:

- Anchor.toml specifies: `0.29.0`
- Actual CLI: `0.31.0`
- Dependencies: `0.29.0`

## Solutions

### Option 1: Remove Existing Anchor Binary and Install 0.29.0 (Recommended)

```bash
# Remove existing anchor binaries
rm -f ~/.avm/bin/anchor
rm -f ~/.cargo/bin/anchor

# Install Anchor 0.29.0
avm install 0.29.0
avm use 0.29.0

# Verify
anchor --version  # Should show 0.29.0

# Now build should work
anchor build
```

### Option 2: Use Build Script (Skip IDL for Now)

```bash
# Use the build script that skips IDL
bash scripts/build-without-idl.sh

# Or via anchor
anchor run build-no-idl
```

### Option 3: Generate IDL After Deployment

After deploying programs, you can generate IDL from the deployed program:

```bash
# Deploy first
anchor deploy

# Then generate IDL from deployed program
anchor idl init -f target/idl/asset_registry.json <PROGRAM_ID>
```

### Option 4: Update to Anchor 0.31.0 (If Compatible)

If you want to use Anchor 0.31.0, update all dependencies:

```bash
# Update Anchor.toml
anchor_version = "0.31.0"

# Update all Cargo.toml files
anchor-lang = "0.31.0"
anchor-spl = "0.31.0"

# Note: You may encounter dependency conflicts (zeroize, solana-instruction)
# See DEPENDENCY_CONFLICT_FIX.md for details
```

## Current Status

✅ **Programs compile successfully** - All three .so files are generated
⚠️ **IDL generation fails** - Due to version mismatch
✅ **Ready for deployment** - IDL is optional for deployment

## Impact

- **Deployment**: ✅ Works fine without IDL
- **Testing**: ⚠️ Some tests may need IDL files
- **Client SDKs**: ⚠️ IDL files needed for TypeScript/JavaScript clients
- **Documentation**: ✅ Can generate IDL later

## Recommendation

For now, you can:

1. Deploy the programs (they work without IDL)
2. Generate IDL files manually after deployment
3. Or fix the Anchor CLI version when you need IDL files

The programs are fully functional - IDL is just metadata for client generation.
