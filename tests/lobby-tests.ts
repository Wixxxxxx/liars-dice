import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { LiarsDice } from "../target/types/liars_dice";
import { PublicKey, Keypair, sendAndConfirmTransaction } from "@solana/web3.js";
import { createGame } from "../test-helpers/createGame";

import { HermesClient } from "@pythnetwork/hermes-client";
import {
  PythSolanaReceiver,
  InstructionWithEphemeralSigners,
} from "@pythnetwork/pyth-solana-receiver";

import { assert } from "chai";

const SOL_PRICE_FEED_ID =
  "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";

describe("lobby-tests", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.LiarsDice as Program<LiarsDice>;
  const host = provider.wallet.publicKey;

  let gamePda: anchor.web3.PublicKey;
  let gameState: anchor.IdlTypes<LiarsDice>["gameState"];

  before(async () => {
    ({ gamePda, gameState } = await createGame(program, provider, 5, 20));
  });

  it("Game PDA created successfully!", async () => {
    // ensure the created game's ID belongs to the host
    console.log("Host Pubkey:", host.toBase58());
    console.log("Game ID:", gameState.gameId.toBase58());
    console.log("Game PDA:", gamePda.toBase58());

    assert.strictEqual(gameState.gameId.toBase58(), host.toBase58());
  });

  it("Player joined successfully!", async () => {
    const player = Keypair.generate();

    // // Airdrop 1 SOL to player
    // const signature = await provider.connection.requestAirdrop(
    //   player.publicKey,
    //   1_000_000_000
    // );

    // const latestBlockhash = await provider.connection.getLatestBlockhash();

    // await provider.connection.confirmTransaction(
    //   {
    //     signature,
    //     blockhash: latestBlockhash.blockhash,
    //     lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    //   },
    //   "confirmed"
    // );

    // build transaction for TwapUpdate
    const hermesClient = new HermesClient("https://hermes.pyth.network/", {});
    const twapWindowSeconds = 300; // 5 minutes

    const twapUpdateData = await hermesClient.getLatestTwaps(
      [SOL_PRICE_FEED_ID], // SOL/USD feed ID
      twapWindowSeconds,
      { encoding: "base64" }
    );

    const pythSolanaReceiver = new PythSolanaReceiver({
      connection: provider.connection,
      wallet: provider.wallet as Wallet,
    });

    const transactionBuilder = pythSolanaReceiver.newTransactionBuilder({
      closeUpdateAccounts: false,
    });

    await transactionBuilder.addPostTwapUpdates(twapUpdateData.binary.data);

    const twapPubkey =
      transactionBuilder.getTwapUpdateAccount(SOL_PRICE_FEED_ID);
    console.log("TWAP account:", twapPubkey.toBase58());

    await transactionBuilder.addTwapConsumerInstructions(
      async (
        getTwapUpdateAccount: (priceFeedId: string) => PublicKey
      ): Promise<InstructionWithEphemeralSigners[]> => {
        return [
          {
            instruction: await program.methods
              .joinGame(gameState.gameId)
              .accounts({
                player: player.publicKey,
                twapUpdate: getTwapUpdateAccount(SOL_PRICE_FEED_ID),
              })
              .instruction(),
            signers: [player],
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

    console.log("Transaction Result:", result);
  });
});
