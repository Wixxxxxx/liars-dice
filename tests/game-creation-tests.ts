import * as anchor from "@coral-xyz/anchor";
import { BN } from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LiarsDice } from "../target/types/liars_dice";
import { assert } from "chai";

const expect = require("chai").expect;

describe("game-creation-tests", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.LiarsDice as Program<LiarsDice>;
  const host = provider.wallet.publicKey;

  it("Game PDA created successfully!", async () => {
    // receive 1 SOL airdrop
    const signature = await provider.connection.requestAirdrop(
      host,
      1000000000
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

    // derive expected pda
    const [game_pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("liarsdicesession"), host.toBuffer()],
      program.programId
    );

    // send initialize instruction
    const player_num = 5;
    const buy_in = 20;

    const tx = await program.methods
      .initialize(new BN(player_num), new BN(buy_in))
      .rpc();

    console.log("Your transaction signature", tx);

    const gameState = await program.account.gameState.fetch(game_pda);
    assert.strictEqual(gameState.gameId.toBase58(), host.toBase58());
  });
});
