use crate::state::game_state::GameState;
use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

pub fn create_game(ctx: Context<CreateGame>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(init, payer = user,space = ANCHOR_DISCRIMINATOR_SIZE + GameState::INIT_SPACE,
            seeds = [b"liarsdicesession", user.key().as_ref()],
            bump)]
    pub game: Account<'info, GameState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
