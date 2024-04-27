use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Mint, Token, TokenAccount};

use crate::state::*;

#[derive(Accounts)]
#[instruction(amount:u64)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub wallet: Signer<'info>,
    #[account(
        seeds = [
            b"record",
            wallet.key().as_ref(),
        ],
        bump,
        mut,
    )]
    pub record: Account<'info, Record>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"mint"],
        bump,
        mut,
    )]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn borrow_(ctx: Context<Borrow>, amount: u64) -> Result<()> {
    //transfer money
    let seeds = &[b"mint".as_ref(),&[ctx.bumps.mint]];
    let signer = [&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.pool_token_account.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint.to_account_info(),
        }, &signer);

    token::transfer(cpi_ctx, amount)?;

    //record the message
    let record = &mut ctx.accounts.record;

    record.init(amount);


    Ok(())
}