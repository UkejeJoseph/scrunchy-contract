# Scrunchy Smart Contracts

Solana Anchor programs for the Scrunchy decentralized game asset marketplace.

## Architecture

This project consists of three Anchor programs:

1. **Asset Registry** (`asset-registry`) - Manages game asset creation, metadata, and ownership
2. **Marketplace** (`marketplace`) - Handles asset listings, rentals, and payment distribution
3. **Access Control** (`access-control`) - Provides fast ownership/rental verification for SDKs

## Prerequisites

See [INSTALLATION.md](../INSTALLATION.md) for complete setup instructions.

Required tools:
- Rust 1.70+
- Solana CLI 1.18+
- Anchor Framework 0.29+
- Node.js 18+

## Quick Start

### 1. Install Dependencies

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

### 2. Build Programs

```bash
anchor build
```

### 3. Run Tests

```bash
anchor test
```

### 4. Deploy to Devnet

```bash
# Set Solana to devnet
solana config set --url devnet

# Get devnet SOL (see docs/AUTO_AIRDROP_GUIDE.md for automated script)
solana airdrop 2

# Deploy programs (or use scripts/deploy-individual.sh for more reliable deployment)
anchor deploy
```

**Note**: For more reliable deployment, use the individual deployment script:
```bash
bash scripts/deploy-individual.sh
```

See [Deployment Guide](./docs/DEPLOY_TO_DEVNET.md) for detailed instructions.

## Program IDs

### Devnet (Placeholder)
- Asset Registry: `AssetRegistry11111111111111111111111111111`
- Marketplace: `Mktplce11111111111111111111111111111111111`
- Access Control: `AccssCtrl111111111111111111111111111111`

**Note**: These are placeholder IDs. After deployment, update `Anchor.toml` with actual program IDs.

## Project Structure

```
scrunchy-contract/
├── programs/                # Solana Anchor programs
│   ├── asset-registry/      # Asset Registry Program
│   ├── marketplace/         # Marketplace Program
│   └── access-control/      # Access Control Program
├── tests/                   # Integration tests
├── scripts/                 # Deployment and utility scripts
├── docs/                    # Project documentation
├── Anchor.toml              # Anchor configuration
└── Cargo.toml              # Workspace configuration
```

## Key Features

### Asset Registry
- Create game assets as NFTs
- Register games for integration
- Manage asset metadata and attributes
- Track asset statistics
- Transfer asset ownership

### Marketplace
- List assets for rent
- Create rental agreements
- Escrow payment management
- Revenue distribution (owner, platform, game)
- Rental extension and completion

### Access Control
- Fast ownership verification
- Rental status checking
- SDK-optimized queries

## Development

### Build Individual Program

```bash
anchor build -p asset-registry
anchor build -p marketplace
anchor build -p access-control
```

### Generate IDL

```bash
anchor idl parse -f programs/asset-registry/src/lib.rs -o target/idl/asset_registry.json
```

### Local Testing

```bash
# Start local validator
solana-test-validator

# In another terminal, run tests
anchor test --skip-local-validator
```

## Security Considerations

- All programs use PDA (Program Derived Address) for account derivation
- Signer validation on all state-changing instructions
- Integer overflow protection with checked arithmetic
- Escrow pattern for secure payment handling
- Cross-program invocation (CPI) validation

## Documentation

### Quick Links
- **[Deployment Guide](./docs/DEPLOY_TO_DEVNET.md)** - Deploy to devnet
- **[Deployment Costs](./docs/DEPLOYMENT_COSTS.md)** - Calculate deployment costs
- **[Troubleshooting](./docs/DEPLOYMENT_ERROR_FIX.md)** - Fix deployment errors
- **[SOL Airdrop Guide](./docs/AUTO_AIRDROP_GUIDE.md)** - Get devnet SOL

### Full Documentation
See the [docs folder](./docs/) for complete documentation including:
- Deployment guides and troubleshooting
- Setup and configuration
- SOL airdrop guides
- Build status and known issues

For detailed architecture documentation, see:
- [Smart Contract Architecture](../docs/01_SCRUNCHY_SMART_CONTRACT_ARCHITECTURE.md)
- [Project Overview](../docs/00_SCRUNCHY_PROJECT_OVERVIEW.md)

## License

MIT License

