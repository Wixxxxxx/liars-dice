use crate::errors::LiarsDiceError;
use crate::game_state::GameState;
use crate::utils::price_utils::convert_usd_to_lamports;
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

            // convert usd buy in to lamports
            let usd_buy_in = self.game.buy_in;
            let amount_to_transfer: u64 =
                convert_usd_to_lamports(usd_buy_in, &mut self.twap_update)?;

            msg!("Placing {} LAMPORTS buy-in in game pot", amount_to_transfer);

            // !!! TODO: transfer funds from player wallet to game state

            Ok(())
        } else {
            Err(LiarsDiceError::GameFull.into())
        }
    }
}
