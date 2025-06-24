use crate::errors::LiarsDiceError;
use crate::game_state::GameState;
use crate::player::Player;
use crate::utils::transfer_utils::transfer_funds;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[derive(Accounts)]
#[instruction(game_id: Pubkey)]
pub struct Join<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [b"liarsdicesession", game_id.as_ref()], bump)]
    pub game: Account<'info, GameState>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut, seeds = [b"vault", game_id.as_ref()], bump)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = player,
        space = ANCHOR_DISCRIMINATOR_SIZE + Player::INIT_SPACE,
        seeds = [b"player", player.key().as_ref()],
        bump)]
    pub guest_player: Account<'info, Player>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = player,
        associated_token::token_program = token_program)]
    pub player_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
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

            let amount = self.game.buy_in_sol;
            let decimals = self.mint.decimals;

            msg!(
                "Player joining session and placing {} LAMPORTS buy-in in game pot",
                amount
            );

            transfer_funds(
                &mut self.player_token_account,
                &mut self.vault,
                &mut self.mint,
                &mut self.player,
                &mut self.token_program,
                amount,
                decimals,
            )
        } else {
            Err(LiarsDiceError::GameFull.into())
        }
    }
}
