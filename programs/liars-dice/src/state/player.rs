use anchor_lang::prelude::*;

#[account]
pub struct Player {
    pub name: String,
    pub game_host: bool,
    pub dice: Vec<u8>,
    pub rounds_won: u64,
    pub rounds_lost: u64,
    pub winnings: Lamports,
}

impl Player {}
