#![allow(unexpected_cfgs)]

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod errors;
pub mod utils;

use anchor_lang::prelude::*;

declare_id!("AZrqTVywNJhAvGvComNiHVbmUCVEyJnhfcKtGURdmG2B");

#[program]
pub mod liars_dice {

    use super::*;
    pub fn initialize(ctx: Context<Start>, player_num: u64, buy_in: u64) -> Result<()> {
        ctx.accounts.create_game(player_num, buy_in)
    }
}
