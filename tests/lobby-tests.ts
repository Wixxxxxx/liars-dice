import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { LiarsDice } from "../target/types/liars_dice";
import { PublicKey, Keypair } from "@solana/web3.js";
import { createGame } from "../test-helpers/createGame";
import { createMint } from "@solana/spl-token";

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
  const host = provider.wallet.payer;

  let gamePda: anchor.web3.PublicKey;
  let gameState: anchor.IdlTypes<LiarsDice>["gameState"];

  before(async () => {
    const mintSOL = await createMint(
      provider.connection,
      provider.wallet.payer,
      host.publicKey,
      null,
      2
    );

    ({ gamePda, gameState } = await createGame(
      program,
      provider,
      5,
      20,
      mintSOL
    ));
  });

  it("Game PDA created successfully!", async () => {
    // ensure the created game's ID belongs to the host
    console.log("Host Pubkey:", host.publicKey.toBase58());
    console.log("Game ID:", gameState.gameId.toBase58());
    console.log("Game PDA:", gamePda.toBase58());

    assert.strictEqual(gameState.gameId.toBase58(), host.publicKey.toBase58());
  });

  it("Player joined successfully!", async () => {});
});
