use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};


#[derive(Accounts)]
pub struct InitializeMintAccount<'info> {
    #[account(
        init,
        seeds = [b"mint"],
        bump,
        payer = wallet,
        mint::decimals = 255,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(
        init,
        payer = wallet,
        token::mint = mint,
        token::authority = mint,
    )]
    pub token_account: Account<'info, TokenAccount>,//Creator's token_account
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_mint(_ctx: Context<InitializeMintAccount>) -> Result<()> {
    Ok(())
}