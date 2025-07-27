use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Player {
    pub player_id: Pubkey,
    #[max_len(5)]
    pub gamer_tag: String,
    pub game_host: bool,
    pub rounds_won: u64,
    pub rounds_lost: u64,
    pub winnings: i64,
    pub is_init: bool,
}

impl Player {
    pub fn init_player(&mut self, player_id: Pubkey, gamer_tag: String, is_game_host: bool) {
        self.player_id = player_id;
        self.gamer_tag = gamer_tag;
        self.game_host = is_game_host;
        self.rounds_won = 0;
        self.rounds_lost = 0;
        self.winnings = 0;
        self.is_init = true;
    }
}
