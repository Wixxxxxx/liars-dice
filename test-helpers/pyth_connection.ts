import * as anchor from "@coral-xyz/anchor";
import { Wallet } from "@coral-xyz/anchor";
import { HermesClient } from "@pythnetwork/hermes-client";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";

const SOL_PRICE_FEED_ID =
  "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";

export async function build_pyth_receiver(
  provider: anchor.AnchorProvider
): Promise<{
  transactionBuilder: ReturnType<PythSolanaReceiver["newTransactionBuilder"]>;
  pythSolanaReceiver: PythSolanaReceiver;
}> {
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

  transactionBuilder.getTwapUpdateAccount(SOL_PRICE_FEED_ID);

  return { pythSolanaReceiver, transactionBuilder };
}
