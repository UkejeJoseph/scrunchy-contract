# Backend IDL Client Setup Guide

## ✅ Yes, You Can Proceed with Backend!

Your IDL files are generated and ready. Here's how to use them in your backend.

## Step 1: Generate TypeScript Clients from IDL

Anchor can automatically generate TypeScript clients from your IDL files. Here's how:

### Option A: Using Anchor CLI (Recommended)

```bash
cd scrunchy-contract

# Generate TypeScript clients from IDL files
anchor idl init --filepath target/idl/asset_registry.json asset_registry
anchor idl init --filepath target/idl/marketplace.json marketplace  
anchor idl init --filepath target/idl/access_control.json access_control
```

### Option B: Manual Client Generation (More Control)

Anchor automatically generates clients when you build. The clients are available via `@coral-xyz/anchor`:

```typescript
import { Program, AnchorProvider } from "@coral-xyz/anchor";
import { Connection, PublicKey } from "@solana/web3.js";
import idl from "./idl/asset_registry.json";

// Create program instance
const programId = new PublicKey("7vxpftgcdbh6NW77uAGVxp4aM2RcbQtmeGixNp2ifCDn");
const program = new Program(idl as any, programId, provider);
```

## Step 2: Copy IDL Files to Backend

```bash
# From project root
mkdir -p scrunchy-backend/src/idl
cp scrunchy-contract/target/idl/*.json scrunchy-backend/src/idl/
```

## Step 3: Backend Setup Structure

Your backend should have this structure:

```
scrunchy-backend/
├── src/
│   ├── idl/                    # IDL files (copied from contract)
│   │   ├── asset_registry.json
│   │   ├── marketplace.json
│   │   └── access_control.json
│   ├── programs/               # TypeScript program clients
│   │   ├── asset-registry.ts
│   │   ├── marketplace.ts
│   │   └── access-control.ts
│   ├── lib/
│   │   ├── solana.ts          # Solana connection setup
│   │   └── programs.ts        # Program initialization
│   └── api/                    # API routes
│       ├── assets/
│       ├── marketplace/
│       └── access/
└── package.json
```

## Step 4: Install Backend Dependencies

```bash
cd scrunchy-backend
npm init -y

# Install required packages
npm install @coral-xyz/anchor@^0.31.0
npm install @solana/web3.js@^1.87.6
npm install dotenv
npm install --save-dev @types/node typescript ts-node
```

## Step 5: Create Program Client Helper

Create `scrunchy-backend/src/lib/programs.ts`:

```typescript
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import assetRegistryIdl from "../idl/asset_registry.json";
import marketplaceIdl from "../idl/marketplace.json";
import accessControlIdl from "../idl/access_control.json";

// Program IDs (from Anchor.toml)
export const PROGRAM_IDS = {
  assetRegistry: new PublicKey("7vxpftgcdbh6NW77uAGVxp4aM2RcbQtmeGixNp2ifCDn"),
  marketplace: new PublicKey("G46EH6LskmJpBTvZNewSL4Xe5fkdbcdkMpRM8wQ9T3RH"),
  accessControl: new PublicKey("n41USxoi9Lf1RyZUVAfCHD5TSeuDBuqDxEpUHzkDqf2"),
};

// Create provider (for read-only operations, no wallet needed)
export function createReadOnlyProvider(connection: Connection): AnchorProvider {
  const wallet = {
    publicKey: PublicKey.default,
    signTransaction: async (tx: any) => tx,
    signAllTransactions: async (txs: any[]) => txs,
  } as Wallet;

  return new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
}

// Create provider with wallet (for write operations)
export function createProvider(
  connection: Connection,
  wallet: Wallet
): AnchorProvider {
  return new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
}

// Initialize program instances
export function getAssetRegistryProgram(provider: AnchorProvider): Program {
  return new Program(
    assetRegistryIdl as any,
    PROGRAM_IDS.assetRegistry,
    provider
  );
}

export function getMarketplaceProgram(provider: AnchorProvider): Program {
  return new Program(
    marketplaceIdl as any,
    PROGRAM_IDS.marketplace,
    provider
  );
}

export function getAccessControlProgram(provider: AnchorProvider): Program {
  return new Program(
    accessControlIdl as any,
    PROGRAM_IDS.accessControl,
    provider
  );
}
```

## Step 6: Example Usage in API Route

Create `scrunchy-backend/src/api/assets/[mint].ts`:

```typescript
import { NextApiRequest, NextApiResponse } from "next";
import { Connection, PublicKey } from "@solana/web3.js";
import {
  createReadOnlyProvider,
  getAssetRegistryProgram,
} from "../../lib/programs";

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method !== "GET") {
    return res.status(405).json({ error: "Method not allowed" });
  }

  const { mint } = req.query;
  
  try {
    // Connect to Solana
    const connection = new Connection(
      process.env.SOLANA_RPC_URL || "https://api.devnet.solana.com",
      "confirmed"
    );

    // Create provider and program
    const provider = createReadOnlyProvider(connection);
    const program = getAssetRegistryProgram(provider);

    // Derive PDA for game asset
    const [gameAssetPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("game_asset"), new PublicKey(mint as string).toBuffer()],
      program.programId
    );

    // Fetch account data
    const gameAsset = await program.account.gameAsset.fetch(gameAssetPda);

    return res.status(200).json({
      mint: gameAsset.mint.toString(),
      owner: gameAsset.owner.toString(),
      name: gameAsset.name,
      assetType: gameAsset.assetType,
      rarity: gameAsset.rarity,
      metadataUri: gameAsset.metadataUri,
      // ... other fields
    });
  } catch (error: any) {
    console.error("Error fetching asset:", error);
    return res.status(500).json({ error: error.message });
  }
}
```

## Step 7: Example - Call Program Instruction

Create `scrunchy-backend/src/api/assets/create.ts`:

```typescript
import { NextApiRequest, NextApiResponse } from "next";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Wallet } from "@coral-xyz/anchor";
import {
  createProvider,
  getAssetRegistryProgram,
} from "../../lib/programs";

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method !== "POST") {
    return res.status(405).json({ error: "Method not allowed" });
  }

  try {
    // Get wallet from environment (server-side keypair)
    const walletKeypair = Keypair.fromSecretKey(
      Buffer.from(JSON.parse(process.env.SERVER_WALLET!))
    );
    const wallet = new Wallet(walletKeypair);

    // Connect to Solana
    const connection = new Connection(
      process.env.SOLANA_RPC_URL || "https://api.devnet.solana.com",
      "confirmed"
    );

    // Create provider and program
    const provider = createProvider(connection, wallet);
    const program = getAssetRegistryProgram(provider);

    const { mint, name, assetType, rarity, metadataUri } = req.body;

    // Derive PDAs
    const mintPubkey = new PublicKey(mint);
    const [gameAssetPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("game_asset"), mintPubkey.toBuffer()],
      program.programId
    );

    // Call instruction
    const tx = await program.methods
      .createAsset(
        name,
        metadataUri,
        assetType,
        rarity,
        [], // baseAttributes
        [], // gameMappings
        null, // rentalConfig
        true, // isRentable
        true // isTransferable
      )
      .accounts({
        authority: wallet.publicKey,
        gameAsset: gameAssetPda,
        mint: mintPubkey,
        // ... other accounts
      })
      .rpc();

    return res.status(200).json({ 
      success: true, 
      transaction: tx,
      gameAsset: gameAssetPda.toString()
    });
  } catch (error: any) {
    console.error("Error creating asset:", error);
    return res.status(500).json({ error: error.message });
  }
}
```

## Step 8: Environment Variables

Create `scrunchy-backend/.env.local`:

```env
# Solana Configuration
SOLANA_RPC_URL=https://api.devnet.solana.com
# Or use Helius/QuickNode for better performance
# SOLANA_RPC_URL=https://rpc.helius.xyz/?api-key=YOUR_KEY

# Server Wallet (for server-side transactions)
# Generate with: solana-keygen new --outfile server-wallet.json
SERVER_WALLET=["your","keypair","array","here"]

# Database (if using)
DATABASE_URL=postgresql://user:password@localhost:5432/scrunchy

# Redis (if using)
REDIS_URL=redis://localhost:6379
```

## Step 9: TypeScript Configuration

Create `scrunchy-backend/tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "lib": ["ES2020"],
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "moduleResolution": "node"
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

## Available Program Methods

Once you have the program instances, you can call any instruction from the IDL:

### Asset Registry
- `program.methods.initialize(...)`
- `program.methods.createAsset(...)`
- `program.methods.updateAsset(...)`
- `program.methods.addGameMapping(...)`
- `program.methods.transferAsset(...)`

### Marketplace
- `program.methods.initializeMarketplace(...)`
- `program.methods.createListing(...)`
- `program.methods.rentAsset(...)`
- `program.methods.completeRental(...)`
- `program.methods.extendRental(...)`

### Access Control
- `program.methods.initializeAccessRecord(...)`
- `program.methods.updateAccessRecord(...)`
- `program.methods.verifyAccess(...)`

## Next Steps

1. ✅ Copy IDL files to backend
2. ✅ Install dependencies
3. ✅ Create program client helpers
4. ✅ Set up API routes
5. ✅ Test with devnet
6. ✅ Deploy to mainnet when ready

## Testing

You can test your setup:

```typescript
// test-program.ts
import { Connection } from "@solana/web3.js";
import { createReadOnlyProvider, getAssetRegistryProgram } from "./lib/programs";

async function test() {
  const connection = new Connection("https://api.devnet.solana.com");
  const provider = createReadOnlyProvider(connection);
  const program = getAssetRegistryProgram(provider);
  
  console.log("Program ID:", program.programId.toString());
  console.log("Program methods:", Object.keys(program.methods));
}

test();
```

## Summary

✅ **You're ready to build the backend!**

The IDL files contain everything needed:
- All instruction signatures
- Account structures
- Type definitions
- Error codes

Just import the IDL JSON files and use `@coral-xyz/anchor`'s `Program` class to interact with your smart contracts!

