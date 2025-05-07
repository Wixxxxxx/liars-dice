use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameState {
    max_players: u64,
    buy_in: ???,

}
