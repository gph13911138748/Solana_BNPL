use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{ Mint, Token, TokenAccount}};

use crate::state::*;

#[derive(Accounts)]
pub struct InitializeAuth<'info> {
    #[account(
        init,
        payer = wallet,
        space = 8 + 32 + 32 + 8 + 1 + 8,
        seeds = [
            b"authority",
            wallet.key().as_ref(),
        ],
        bump,
    )]
    pub authority: Account<'info,Authority>, //record investors's information
    #[account(mut)]
    pub wallet: Signer<'info>,
    #[account(
        seeds = [
            b"mint"
        ],
        bump,
        mut,
    )]
    pub mint: Account<'info,Mint>,
    #[account(mut)]//need to offer to the public
    pub pool_token_account: Account<'info,TokenAccount>,//pool's token account
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,//investors token_account
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info,System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    #[account(
        seeds = [
            b"vault",
        ],
        bump,
        mut,
    )]
    pub vault: Account<'info, Vault>,
}

pub fn  initialize_auth(ctx: Context<InitializeAuth>) -> Result<()> {
    let authority = &mut ctx.accounts.authority;

    authority.token_account = ctx.accounts.pool_token_account.key();
    authority.wallet = ctx.accounts.wallet.key();
    authority.amount = 0;
    authority.stake_at = None;

    Ok(())
}