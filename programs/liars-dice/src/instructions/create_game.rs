use crate::state::game_state::GameState;
use crate::utils::price_utils::convert_usd_to_lamports;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use pyth_solana_receiver_sdk::price_update::TwapUpdate;

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[derive(Accounts)]
pub struct Start<'info> {
    #[account(mut)]
    pub host: Signer<'info>,
    #[account(
        init_if_needed,
        payer = host,
        space = ANCHOR_DISCRIMINATOR_SIZE + GameState::SIZE,
        seeds = [b"liarsdicesession", host.key().as_ref()],
        bump)]
    pub game: Account<'info, GameState>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        token::mint = mint,
        token::authority = game,
        payer = host,
        seeds = [b"vault", host.key().as_ref()],
        bump)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = host,
        associated_token::token_program = token_program)]
    pub host_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub twap_update: Account<'info, TwapUpdate>,
}

impl<'info> Start<'info> {
    pub fn create_game(&mut self, player_num: u64, buy_in_usd: u64) -> Result<()> {
        msg!(
            "Creating game with {} players and ${} buy-in",
            player_num,
            buy_in_usd
        );

        let game_id = self.host.key();
        let decimals = self.mint.decimals;
        let buy_in_sol = convert_usd_to_lamports(buy_in_usd, decimals, &mut self.twap_update)?;
        self.game
            .initialize_game(game_id, player_num, buy_in_usd, buy_in_sol)?;

        msg!(
            "Game session created successfully with ID: {}! and buy-in {} LAMPORTS",
            game_id,
            buy_in_sol
        );

        let pot_deposit_cpi_accounts = TransferChecked {
            from: self.host_token_account.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.host.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, pot_deposit_cpi_accounts);

        transfer_checked(cpi_ctx, buy_in_sol, decimals)?;

        msg!("Host deposited {} SOL into the game pot", buy_in_sol);

        Ok(())
    }
}
