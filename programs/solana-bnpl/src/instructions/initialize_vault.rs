use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{ Mint, Token, TokenAccount}};

use crate::state::*;

#[derive(Accounts)]
pub struct InitializeVault<'info> { // build a pool
    #[account(
        init,
        payer = wallet,
        space = 8 + 32 + 32,
        seeds = [
            b"vault",
        ],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        init,
        payer = wallet,//common wallet for deploy
        token::mint = mint,
        token::authority = mint,
    )]
    pub token_account: Account<'info, TokenAccount>,//pool's account
    #[account(
        seeds = [b"mint"],
        bump,
        mut,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_vault_(ctx: Context<InitializeVault>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    vault.token_account = ctx.accounts.token_account.key();
    vault.mint = ctx.accounts.mint.key();

    Ok(())
}