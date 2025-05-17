use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Player {
    #[max_len(5)]
    pub name: String,
    pub game_host: bool,
    #[max_len(5)]
    pub dice: Vec<u8>,
    pub rounds_won: u64,
    pub rounds_lost: u64,
    pub winnings: u64,
}

impl Player {}
