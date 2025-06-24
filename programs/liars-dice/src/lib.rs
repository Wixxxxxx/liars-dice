#![allow(unexpected_cfgs)]

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod errors;
pub mod utils;

use anchor_lang::prelude::*;

declare_id!("3TgXhxUPSweZi49FSByfXAXXxnvoFJaui9kaZxfuUTSu");

#[program]
pub mod liars_dice {

    use super::*;
    pub fn initialize_game(
        ctx: Context<Start>,
        player_num: u64,
        buy_in: u64,
        gamer_tag: String,
    ) -> Result<()> {
        ctx.accounts.create_game(player_num, buy_in, gamer_tag)
    }

    pub fn join_game(ctx: Context<Join>, game_id: Pubkey) -> Result<()> {
        ctx.accounts.join_game(game_id)
    }
}
