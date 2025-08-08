#![allow(unexpected_cfgs)]

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod errors;
pub mod utils;

use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::{delegate, ephemeral};
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_vrf_sdk::consts::{DEFAULT_EPHEMERAL_QUEUE, VRF_PROGRAM_IDENTITY};
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};

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
        msg!("Initializing game session...");
        ctx.accounts.create_game(player_num, buy_in, gamer_tag)
    }

    pub fn join_game(ctx: Context<Join>, game_id: Pubkey) -> Result<()> {
        ctx.accounts.join_game(game_id)
    }

    pub fn request_randomness(ctx: Context<RollDiceDelegatedCtx>) -> Result<()> {
        // request randomness for dice generation
        msg!("Requesting randomness...");
        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: ctx.accounts.payer.key(),
            oracle_queue: ctx.accounts.oracle_queue.key(),
            callback_program_id: ID,
            callback_discriminator: instruction::CallbackRollDiceSimple::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed; 32],
            accounts_metas: None,
            ..Default::default()
        });
        ctx.accounts
            .invoke_signed_vrf(&ctx.accounts.payer.to_account_info(), &ix)?;
    }

    pub fn run_game(ctx: Context<Run>) -> Result<()> {
        // delegate game
        ctx.accounts.delegate_temp(
            &ctx.accounts.host,
            &[b"liarsdicesession", ctx.accounts.host.key().as_ref()],
            DelegateConfig::default(),
        )?;

        launch(&mut ctx.accounts.game)?;

        Ok(())
    }
}

#[vrf]
#[delegate]
#[derive(Accounts)]
pub struct Run<'info> {
    pub host: Signer<'info>,
    #[account(mut, del)]
    pub temp: AccountInfo<'info>,
    #[account(mut, seeds = [b"liarsdicesession", host.key().as_ref()], bump)]
    pub game: Account<'info, GameState>,
    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,
    #[account(mut, address = DEFAULT_EPHEMERAL_QUEUE)]
    pub oracle_queue: AccountInfo<'info>,
}
