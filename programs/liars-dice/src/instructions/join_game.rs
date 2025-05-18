use crate::errors::LiarsDiceError;
use crate::game_state::GameState;
use crate::utils::price_utils::convert_usd_to_lamports;
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::TwapUpdate;

#[derive(Accounts)]
#[instruction(game_id: Pubkey)]
pub struct Join<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [b"liarsdicesession", game_id.as_ref()], bump)]
    pub game: Account<'info, GameState>,
    pub twap_update: Account<'info, TwapUpdate>,
}

impl<'info> Join<'info> {
    pub fn join_game(&mut self, game_id: Pubkey) -> Result<()> {
        // make sure player joining with valid game id
        require!(self.game.game_id == game_id, LiarsDiceError::InvalidGameId);

        // make sure player hasn't already joined
        require!(
            !self
                .game
                .players
                .iter()
                .any(|p| *p == Some(self.player.key())),
            LiarsDiceError::PlayerAlreadyJoined
        );

        // find first empty slot and insert
        if let Some(slot) = self.game.players.iter_mut().find(|p| p.is_none()) {
            *slot = Some(self.player.key());

            // convert usd buy in to lamports
            let usd_buy_in = self.game.buy_in;
            let amount_to_transfer: u64 =
                convert_usd_to_lamports(usd_buy_in, &mut self.twap_update)?;

            msg!("Placing {} LAMPORT buy-in in game pot", amount_to_transfer);

            // !!! TODO: transfer funds from player wallet to game state

            Ok(())
        } else {
            Err(LiarsDiceError::GameFull.into())
        }
    }
}

// #[cfg(test)]
// pub mod test {
//     use super::*;
//     use crate::accounts::Initialize as InitializeAccounts;
//     use crate::instruction::Initialize as InitializeInstruction;
//     use anchor_lang::{InstructionData, ToAccountMetas};
//     use litesvm::LiteSVM;
//     use solana_sdk::{
//         instruction::{AccountMeta, Instruction},
//         message::Message,
//         pubkey::Pubkey,
//         signature::Keypair,
//         signature::Signer,
//         transaction::Transaction,
//     };

//     #[tokio::test]
//     pub async fn test_basic_program() {
//         // 1. start up new SVM instance (local validator)
//         let mut svm = LiteSVM::new();

//         // 2. generate payer keypair to sign
//         let host_kp = Keypair::new();
//         let host_pk = host_kp.pubkey();

//         // 3. read in program binary and airdop sol to payer
//         let program_id = pubkey!("AZrqTVywNJhAvGvComNiHVbmUCVEyJnhfcKtGURdmG2B");
//         let bytes = include_bytes!("../../../../target/deploy/liars_dice.so");
//         svm.add_program(program_id, bytes);
//         svm.airdrop(&host_pk, 1_000_000_000).unwrap();

//         // 4. create instruction
//         let accounts = InitializeAccounts {}.to_account_metas(Some(false));
//         let data = InitializeInstruction.data();
//         let ix = Instruction {
//             program_id,
//             accounts,
//             data,
//         };

//         // 5. create message and send transaction
//         let msg = Message::new_with_blockhash(&[ix], Some(&host_pk), &svm.latest_blockhash());
//         let tx = Transaction::new(&[host_kp], msg, svm.latest_blockhash());
//         let meta = svm.send_transaction(tx).unwrap();
//         assert_eq!(
//             meta.logs[2],
//             "Program log: Greetings from: AZrqTVywNJhAvGvComNiHVbmUCVEyJnhfcKtGURdmG2B"
//         )
//     }
// }
