use anchor_lang::error_code;

#[error_code]
pub enum LiarsDiceError {
    #[msg("Maximum number of players is 5")]
    MaxPlayerNumExceeded,
    #[msg("Minimum number of players is 3")]
    MinPlayerNumNotReached,
    #[msg("Invalid game ID")]
    InvalidGameId,
    #[msg("Game session is full")]
    GameFull,
    #[msg("Player already joined the game")]
    PlayerAlreadyJoined,
}
