import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

export async function airdropSol(
  provider: anchor.AnchorProvider,
  recipient: PublicKey
) {
  const signature = await provider.connection.requestAirdrop(
    recipient,
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
}
