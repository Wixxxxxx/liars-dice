use crate::state::game_state::GameState;
use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[derive(Accounts)]
pub struct Start<'info> {
    #[account(init, payer = host,space = ANCHOR_DISCRIMINATOR_SIZE + GameState::SIZE,
            seeds = [b"liarsdicesession", host.key().as_ref()],
            bump)]
    pub game: Account<'info, GameState>,
    #[account(mut)]
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Start<'info> {
    pub fn create_game(&mut self, player_num: u64, buy_in: u64) -> Result<()> {
        msg!(
            "Creating game with {} players and {} lamports buy-in",
            player_num,
            buy_in
        );
        let game_id = self.host.key();
        self.game.initialize_game(game_id, player_num, buy_in)?;
        msg!("Game session created successfully with ID: {}!", game_id);
        Ok(())
    }
}

// TODO: rewrite tests with updated create game function
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
