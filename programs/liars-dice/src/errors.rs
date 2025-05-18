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

#[error_code]
pub enum PriceConversionError {
    #[msg("Multiplication overflow")]
    MultOverflow,
    #[msg("Division Error")]
    DivError,
    #[msg("Conversion result is too large")]
    ResultTooLarge,
    #[msg("Invalid exponent")]
    InvalidExponent,
    #[msg("Invalid feed")]
    FeedError,
    #[msg("Clock error")]
    ClockError,
    #[msg("Price fetch error")]
    PriceFetchError,
}
