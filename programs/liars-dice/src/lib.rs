#![allow(unexpected_cfgs)]

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod errors;
pub mod utils;

use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::{delegate, ephemeral};

declare_id!("3TgXhxUPSweZi49FSByfXAXXxnvoFJaui9kaZxfuUTSu");

#[ephemeral]
#[program]
pub mod liars_dice {

    use ephemeral_rollups_sdk::cpi::DelegateConfig;

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

    pub fn run_game(ctx: Context<Run>) -> Result<()> {
        // delegate game and call kickstart loop
        ctx.accounts.delegate_temp(
            &ctx.accounts.host,
            &[b"liarsdicesession", ctx.accounts.host.key().as_ref()],
            DelegateConfig::default(),
        )?;

        launch(&mut ctx.accounts.game)?;

        Ok(())
    }
}

#[delegate]
#[derive(Accounts)]
pub struct Run<'info> {
    pub host: Signer<'info>,
    #[account(mut, del)]
    pub temp: AccountInfo<'info>,
    #[account(mut, seeds = [b"liarsdicesession", host.key().as_ref()], bump)]
    pub game: Account<'info, GameState>,
}
