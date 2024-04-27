use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Mint, Token, TokenAccount};

use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct Back<'info> {
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

pub fn back_(ctx: Context<Back>) -> Result<()> {
    //transfer money
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.token_account.to_account_info(),
            to: ctx.accounts.pool_token_account.to_account_info(),
            authority: ctx.accounts.wallet.to_account_info(),
        });

    //get record account
    let record = &mut ctx.accounts.record;

    let (add_tokens, _current_time) = calc_interest(record.borrow_at.unwrap(), 1)?;
    token::transfer(cpi_ctx, add_tokens + record.amount)?;

    //record the message
    record.amount = 0;
    record.borrow_at = None;

    Ok(())
}