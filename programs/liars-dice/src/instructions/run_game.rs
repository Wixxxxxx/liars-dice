use crate::errors::LiarsDiceError;
use crate::state::game_state::{GamePhase, GameState};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(game_id: Pubkey)]
pub struct Run<'info> {
    #[account(
        mut,
        seeds = [b"liarsdicesession", game_id.key().as_ref()],
        bump)]
    pub game: Account<'info, GameState>,
}

impl<'info> Run<'info> {
    pub fn launch(&mut self) -> Result<()> {
        if self.game.is_ready || self.game.players.len() == 5 {
            self.game.game_phase = GamePhase::RoundStart;
            Ok(())
        } else {
            Err(LiarsDiceError::GameNotReady.into())
        }
    }

    pub fn game_loop(&mut self) -> Result<()> {}
}
