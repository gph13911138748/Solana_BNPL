use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

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
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,//it is built by a common wallet, it's address is public
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
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

    msg!("initialize a token pool");
    Ok(())
}