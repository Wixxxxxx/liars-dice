import * as anchor from "@coral-xyz/anchor";
import { BN } from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LiarsDice } from "../../target/types/liars_dice";

export async function createGame(
  program: Program<LiarsDice>,
  provider: anchor.AnchorProvider,
  playerNum: number,
  buyIn: number
): Promise<{ gamePda: anchor.web3.PublicKey; gameState: any; tx: string }> {
  const host = provider.wallet.publicKey;

  // Airdrop 1 SOL
  const signature = await provider.connection.requestAirdrop(
    host,
    1_000_000_000
  );

  const latestBlockhash = await provider.connection.getLatestBlockhash();

  await provider.connection.confirmTransaction(
    {
      signature,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    },
    "confirmed"
  );

  // Derive PDA
  const [gamePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("liarsdicesession"), host.toBuffer()],
    program.programId
  );

  // Initialize game
  const tx = await program.methods
    .initialize(new BN(playerNum), new BN(buyIn))
    .rpc();

  const gameState = await program.account.gameState.fetch(gamePda);

  return { gamePda, gameState, tx };
}
