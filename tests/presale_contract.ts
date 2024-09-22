import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PresaleContract } from "../target/types/presale_contract";
import {ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo} from "@solana/spl-token"

describe("presale_contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PresaleContract as Program<PresaleContract>;

  it("Is initialized!", async () => {
    // Add your test here.
    let token_

    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
