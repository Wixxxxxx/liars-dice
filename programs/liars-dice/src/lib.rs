#![allow(unexpected_cfgs)]

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod errors;
pub mod utils;

use anchor_lang::prelude::*;

declare_id!("G7RC2ZVv9eyxAcZeKwATSKx8X822KgnQ3og5gA1MAjZT");

#[program]
pub mod liars_dice {

    use super::*;
    pub fn initialize(ctx: Context<Start>, player_num: u64, buy_in: u64) -> Result<()> {
        ctx.accounts.create_game(player_num, buy_in)
    }

    pub fn join_game(ctx: Context<Join>, game_id: Pubkey) -> Result<()> {
        ctx.accounts.join_game(game_id)
    }
}
