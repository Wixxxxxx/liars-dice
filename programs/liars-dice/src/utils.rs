pub mod price_utils {
    use crate::errors::PriceConversionError;
    use anchor_lang::prelude::{msg, Clock, SolanaSysvar};
    use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, TwapUpdate};

    pub const FEED_ID: &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
    pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

    pub fn convert_usd_to_lamports(
        amount_in_usd: u64,
        decimals: u8,
        twap_update: &mut TwapUpdate,
    ) -> Result<u64, PriceConversionError> {
        // connect to pyth SOL/USD feed
        let feed_id: [u8; 32] =
            get_feed_id_from_hex(FEED_ID).map_err(|_| PriceConversionError::FeedError)?;

        msg!("The actual feed ID is: {:?}", feed_id);
        msg!("The feed ID passed in is: {:?}", twap_update.twap.feed_id);

        // fetch up to date SOL price
        let sol_price = twap_update
            .get_twap_no_older_than(
                &Clock::get().map_err(|_| PriceConversionError::ClockError)?,
                30,
                300,
                &feed_id,
            )
            .map_err(|_| PriceConversionError::PriceFetchError)?;

        let scale: u32 = sol_price
            .exponent
            .saturating_abs()
            .try_into()
            .map_err(|_| PriceConversionError::InvalidExponent)?;

        let units_per_token = 10_u128.pow(decimals as u32);

        let amount_in_lamports = (units_per_token)
            .checked_mul(10_u128.pow(scale))
            .ok_or(PriceConversionError::MultOverflow)?
            .checked_mul(amount_in_usd as u128)
            .ok_or(PriceConversionError::MultOverflow)?
            .checked_div(sol_price.price as u128)
            .ok_or(PriceConversionError::DivError)?;

        amount_in_lamports
            .try_into()
            .map_err(|_| PriceConversionError::ResultTooLarge)
    }
}

pub mod transfer_utils {
    use crate::errors::TransferError;
    use anchor_lang::{
        error::Error,
        prelude::{CpiContext, Interface, InterfaceAccount, Signer, ToAccountInfo},
    };
    use anchor_spl::{
        token_2022::{transfer_checked, TransferChecked},
        token_interface::{Mint, TokenAccount, TokenInterface},
    };

    pub fn transfer_funds<'a>(
        from_acct: &mut InterfaceAccount<'a, TokenAccount>,
        to_act: &mut InterfaceAccount<'a, TokenAccount>,
        mint: &mut InterfaceAccount<'a, Mint>,
        authority: &mut Signer<'a>,
        token_program: &mut Interface<'a, TokenInterface>,
        amount: u64,
        decimals: u8,
    ) -> Result<(), Error> {
        let pot_deposit_cpi_accounts = TransferChecked {
            from: from_acct.to_account_info(),
            to: to_act.to_account_info(),
            mint: mint.to_account_info(),
            authority: authority.to_account_info(),
        };

        let cpi_program = token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, pot_deposit_cpi_accounts);

        transfer_checked(cpi_ctx, amount, decimals).map_err(|_| TransferError::TransferError)?;

        Ok(())
    }
}
