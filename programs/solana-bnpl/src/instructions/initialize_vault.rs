use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use crate::state::*;
use crate::instructions::*;

#[derive(Accounts)]
pub struct InitializeVault<'info> { // build a pool
    #[account(
        init,
        payer = wallet,
        space = 8 + 32 + 32 + 1 + 8,
        seeds = [
            b"vault",
        ],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        init,
        payer = wallet,
        token::mint = mint,
        token::authority = vault,
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}