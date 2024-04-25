use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

use crate::state::*;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        seeds = [b"mint"],
        bump,
        mut,
    )]
    pub mint: Account<'info,Mint>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info,System>,
    #[account(
        seeds = [
            b"vault",
        ],
        bump,
        has_one = token_account,
        has_one = mint,
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        init_if_needed,
        payer = wallet,
        token::mint = mint,
        token::authority = vault,
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub rent: Sysvar<'info, Rent>
}

pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            }, 
            &[&[b"mint".as_ref(), &[ctx.bumps.mint]]],//do not use 'get'
        ), 
        100000)?;
    Ok(())
}