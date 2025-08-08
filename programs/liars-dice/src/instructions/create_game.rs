use crate::utils::price_utils::convert_usd_to_lamports;
use crate::utils::transfer_utils::transfer_funds;
use crate::{state::game_state::GameState, Player};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use pyth_solana_receiver_sdk::price_update::TwapUpdate;

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[derive(Accounts)]
pub struct Start<'info> {
    #[account(mut)]
    pub host: Signer<'info>,
    #[account(
        init_if_needed,
        payer = host,
        space = ANCHOR_DISCRIMINATOR_SIZE + Player::INIT_SPACE,
        seeds = [b"player", host.key().as_ref()],
        bump)]
    host_player: Account<'info, Player>,
    #[account(
        init_if_needed,
        payer = host,
        space = ANCHOR_DISCRIMINATOR_SIZE + GameState::SIZE,
        seeds = [b"liarsdicesession", host.key().as_ref()],
        bump)]
    pub game: Account<'info, GameState>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        token::mint = mint,
        token::authority = game,
        payer = host,
        seeds = [b"vault", host.key().as_ref()],
        bump)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = host,
        associated_token::token_program = token_program)]
    pub host_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub twap_update: Account<'info, TwapUpdate>,
}

impl<'info> Start<'info> {
    pub fn create_game(
        &mut self,
        player_num: u64,
        buy_in_usd: u64,
        gamer_tag: String,
    ) -> Result<()> {
        msg!(
            "Creating game with {} players and ${} buy-in",
            player_num,
            buy_in_usd
        );

        let game_id = self.host.key();
        let decimals = self.mint.decimals;
        let buy_in_sol = convert_usd_to_lamports(buy_in_usd, decimals, &mut self.twap_update)?;

        self.game
            .initialize_game(game_id, player_num, buy_in_usd, buy_in_sol)?;

        msg!("Inserting host to player list...");

        // insert host to player list and update host player PDA
        self.game.players[0] = Some(self.host.key());
        if !self.host_player.is_init {
            self.host_player
                .init_player(self.host.key(), gamer_tag, true);
        }

        msg!(
            "Transferring host ${}, {} SOL buy-in to game pot",
            buy_in_usd,
            buy_in_sol
        );

        transfer_funds(
            &mut self.host_token_account,
            &mut self.vault,
            &mut self.mint,
            &mut self.host,
            &mut self.token_program,
            buy_in_sol,
            decimals,
        )
    }
}
