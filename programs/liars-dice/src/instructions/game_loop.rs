use crate::errors::LiarsDiceError;
use crate::state::game_state::{GamePhase, GameState};
use anchor_lang::prelude::*;

pub fn launch(game: &mut GameState) -> Result<()> {
    if game.is_ready || game.players.len() == 5 {
        game.game_phase = GamePhase::RoundStart;
        game_loop(game)?;
        Ok(())
    } else {
        Err(LiarsDiceError::GameNotReady.into())
    }
}

pub fn game_loop(game: &mut GameState) -> Result<()> {
    game.game_phase = GamePhase::RoundOngoing;

    // need to run VRF 5 times for each player

    Ok(())
}
