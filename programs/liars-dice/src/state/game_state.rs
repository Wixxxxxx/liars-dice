use crate::errors::LiarsDiceError;
use anchor_lang::prelude::*;

#[account]
pub struct GameState {
    pub game_id: Pubkey,
    pub players: [Option<Pubkey>; 5],
    pub game_phase: GamePhase,
    pub buy_in_usd: u64,
    pub buy_in_sol: u64,
    pub token_mint: Pubkey, // !don't forget to init
    pub pot: u64,
    pub is_ready: bool,
}

impl GameState {
    pub const MAX_PLAYERS: usize = 5;
    pub const OPTION_PUBKEY_SIZE: usize = 1 + 32;
    // !!! NEED TO CHANGE SIZE FOR ARRAY
    pub const SIZE: usize = 32 + 4 + Self::OPTION_PUBKEY_SIZE * Self::MAX_PLAYERS + 1 + 8 + 8;

    pub fn initialize_game(
        &mut self,
        game_id: Pubkey,
        player_num: u64,
        buy_in_usd: u64,
        buy_in_sol: u64,
    ) -> Result<()> {
        require!(player_num >= 3, LiarsDiceError::MinPlayerNumNotReached);
        require!(player_num <= 5, LiarsDiceError::MaxPlayerNumExceeded);
        self.game_id = game_id;
        self.players = [None; 5];
        self.players[0] = Some(game_id);
        self.game_phase = GamePhase::Lobby;
        self.buy_in_usd = buy_in_usd;
        self.buy_in_sol = buy_in_sol;
        self.pot = buy_in_sol;
        self.is_ready = false;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GamePhase {
    Lobby,
    RoundStart,
    RoundOngoing,
    RoundEnd,
    GameEnd,
}
