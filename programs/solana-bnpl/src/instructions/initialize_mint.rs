use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};


#[derive(Accounts)]
pub struct InitializeMintAccount<'info> {
    #[account(
        init,
        seeds = [b"mint"],
        bump,
        payer = wallet,
        mint::decimals = 9,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_mint(_ctx: Context<InitializeMintAccount>) -> Result<()> {
    msg!("Token mint initialized");
    Ok(())
}