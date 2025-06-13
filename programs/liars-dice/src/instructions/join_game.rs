use crate::errors::LiarsDiceError;
use crate::game_state::GameState;
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::TwapUpdate;

#[derive(Accounts)]
#[instruction(game_id: Pubkey)]
pub struct Join<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut, seeds = [b"liarsdicesession", game_id.as_ref()], bump)]
    pub game: Account<'info, GameState>,
    pub twap_update: Account<'info, TwapUpdate>,
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

            // msg!("Placing {} LAMPORTS buy-in in game pot", amount_to_transfer);

            // !!! TODO: transfer funds from player wallet to game state (WE NOW HAVE THE CORRECT AMOUNT)

            Ok(())
        } else {
            Err(LiarsDiceError::GameFull.into())
        }
    }
}
