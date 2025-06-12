use crate::state::game_state::GameState;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use pyth_solana_receiver_sdk::price_update::TwapUpdate;

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[derive(Accounts)]
pub struct Start<'info> {
    #[account(mut)]
    pub host: Signer<'info>,
    #[account(init_if_needed, payer = host,space = ANCHOR_DISCRIMINATOR_SIZE + GameState::SIZE,
            seeds = [b"liarsdicesession", host.key().as_ref()],
            bump)]
    pub game: Account<'info, GameState>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(init_if_needed, token::mint = mint, token::authority = vault, payer = host, seeds = [b"vault", host.key().as_ref()], bump)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub host_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub twap_update: Account<'info, TwapUpdate>,
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

        // transfer sol from host to game pot --> rewrite this into utils to reuse for join game

        Ok(())
    }
}
