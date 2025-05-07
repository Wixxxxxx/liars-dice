#![allow(unexpected_cfgs)]

pub mod errors;
pub mod instructions;
pub mod state;
use anchor_lang::prelude::*;

declare_id!("AZrqTVywNJhAvGvComNiHVbmUCVEyJnhfcKtGURdmG2B");

#[program]
pub mod liars_dice {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct GameState {}
