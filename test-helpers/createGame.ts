import * as anchor from "@coral-xyz/anchor";
import { BN } from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LiarsDice } from "../target/types/liars_dice";
import { PublicKey, Keypair } from "@solana/web3.js";
import {
  getAssociatedTokenAddress,
  mintTo,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";

import { InstructionWithEphemeralSigners } from "@pythnetwork/pyth-solana-receiver";
import { build_pyth_receiver } from "./pyth_connection";
import { sha256 } from "@coral-xyz/anchor/dist/cjs/utils";

const SOL_PRICE_FEED_ID =
  "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";

export async function createGame(
  program: Program<LiarsDice>,
  provider: anchor.AnchorProvider,
  playerNum: number,
  buyIn: number,
  mint: anchor.web3.PublicKey
): Promise<{
  gamePda: anchor.web3.PublicKey;
  gameState: anchor.IdlTypes<LiarsDice>["gameState"];
}> {
  const host = provider.wallet.payer;

  // derive game PDA
  const [gamePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("liarsdicesession"), host.publicKey.toBuffer()],
    program.programId
  );

  // derive vault
  const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), host.publicKey.toBuffer()],
    program.programId
  );

  const hostTokenAccount = await getAssociatedTokenAddress(
    mint,
    host.publicKey
  );

  const ataIx = createAssociatedTokenAccountInstruction(
    host.publicKey,
    hostTokenAccount,
    host.publicKey,
    mint
  );

  const ataTx = new anchor.web3.Transaction().add(ataIx);
  await provider.sendAndConfirm(ataTx, [host]);

  await mintTo(
    provider.connection,
    host,
    mint,
    hostTokenAccount,
    host,
    buyIn * 5
  );

  const tokenProgram = anchor.utils.token.TOKEN_PROGRAM_ID;

  const { pythSolanaReceiver, transactionBuilder } = await build_pyth_receiver(
    provider
  );

  await transactionBuilder.addTwapConsumerInstructions(
    async (
      getTwapUpdateAccount: (priceFeedId: string) => PublicKey
    ): Promise<InstructionWithEphemeralSigners[]> => {
      return [
        {
          instruction: await program.methods
            .initializeGame(new BN(playerNum), new BN(buyIn))
            .accounts({
              host: host.publicKey,
              mint: mint,
              tokenProgram: tokenProgram,
              twapUpdate: getTwapUpdateAccount(SOL_PRICE_FEED_ID),
            })
            .instruction(),
          signers: [host],
        },
      ];
    }
  );

  const tx = await transactionBuilder.buildVersionedTransactions({
    computeUnitPriceMicroLamports: 100_000,
  });

  // Send the instructions
  const result = await pythSolanaReceiver.provider.sendAll(tx, {
    skipPreflight: true,
  });

  const gameState = await program.account.gameState.fetch(gamePda);

  return { gamePda, gameState };
}
