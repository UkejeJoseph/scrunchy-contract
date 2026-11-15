import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

describe("scrunchy-contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Add your program IDs here after deployment
  // const assetRegistryProgram = anchor.workspace.AssetRegistry;
  // const marketplaceProgram = anchor.workspace.Marketplace;
  // const accessControlProgram = anchor.workspace.AccessControl;

  it("Is initialized!", async () => {
    // Add your tests here
    // Example:
    // const tx = await assetRegistryProgram.methods
    //   .initialize()
    //   .accounts({
    //     authority: provider.wallet.publicKey,
    //     config: configKeypair.publicKey,
    //     systemProgram: SystemProgram.programId,
    //   })
    //   .rpc();
    
    // console.log("Your transaction signature", tx);
  });
});

