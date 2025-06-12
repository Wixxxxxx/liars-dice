// import * as anchor from "@coral-xyz/anchor";
// import { PublicKey } from "@solana/web3.js";
// import { Program } from "@coral-xyz/anchor";
// import { LiarsDice } from "../target/types/liars_dice";
// import { createGame } from "../test-helpers/createGame";

// import { assert } from "chai";
// import {
//   PythSolanaReceiver,
//   InstructionWithEphemeralSigners,
// } from "@pythnetwork/pyth-solana-receiver";

// const expect = require("chai").expect;

// describe("game-join-tests", () => {
//   // Configure the client to use the local cluster.
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

//   const program = anchor.workspace.LiarsDice as Program<LiarsDice>;

//   it("Game joined successfully!", async () => {
//     // create new game to join and initialize player
//     const { gamePda, gameState } = await createGame(program, provider, 5, 20);
//     const player = PublicKey.unique();

//     // build transaction for TwapUpdate
//     const hermesClient = new HermesClient("https://hermes.pyth.network/", {});
//     const twapWindowSeconds = 300; // 5 minutes

//     const twapUpdateData = await hermesClient.getLatestTwaps(
//       ["0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d"], // SOL/USD feed ID
//       twapWindowSeconds,
//       { encoding: "base64" }
//     );

//     console.log(twapUpdateData.binary.data);

//     const pythSolanaReceiver = new PythSolanaReceiver({
//       connection: provider.connection,
//       wallet:
//         provider.wallet as unknown as import("@pythnetwork/pyth-solana-receiver/node_modules/@coral-xyz/anchor").Wallet,
//     });

//     const transactionBuilder = pythSolanaReceiver.newTransactionBuilder({
//       closeUpdateAccounts: false,
//     });

//     await transactionBuilder.addPostTwapUpdates(twapUpdateData.binary.data);

//     await transactionBuilder.addTwapConsumerInstructions(
//       async (
//         getTwapUpdateAccount: (priceFeedId: string) => PublicKey
//       ): Promise<InstructionWithEphemeralSigners[]> => {
//         // Generate instructions here that use the TWAP updates posted above
//         // getTwapUpdateAccount(<price feed id>) will give you the account for each TWAP update
//         //
//         const ix = await program.methods
//           .joinGame(gameState.gameId)
//           .accounts({
//             player: player,
//             twapUpdate: getTwapUpdateAccount(
//               "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d"
//             ),
//           })
//           .instruction();

//         return [];
//       }
//     );

//     // Send the instructions
//     const result = await pythSolanaReceiver.provider.sendAll(
//       await transactionBuilder.buildVersionedTransactions({
//         computeUnitPriceMicroLamports: 50000,
//       }),
//       { skipPreflight: true }
//     );

//     console.log(result);
//   });
// });
