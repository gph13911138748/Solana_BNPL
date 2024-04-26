use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{self, Token, TokenAccount}};

use crate::state::*;

#[derive(Accounts)]
#[instruction(amount:i64)]
pub struct Stake<'info> {
    #[account(mut)]
    pub wallet: Signer<'info>,
    #[account(
        seeds = [
            b"vault",
        ],
        bump,
        mut,
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        seeds = [
            b"authority",
            wallet.key().as_ref(),
        ],
        bump,
        has_one = wallet,
        constraint = authority.token_account == vault.token_account,
        mut,
    )]
    pub authority: Account<'info, Authority>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>, //userAta account controlled by wallet
    #[account(mut)]
    pub token_account_pool: Account<'info, TokenAccount>, //pool's public address
    pub token_program: Program<'info,Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info,System>,
}

pub fn stake_(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.token_account.to_account_info(),
            to: ctx.accounts.token_account_pool.to_account_info(),
            authority: ctx.accounts.wallet.to_account_info(),
        }
    );

    token::transfer(cpi_ctx, amount)?;
    ctx.accounts.authority.init(amount);

    msg!("stake {} tokens successfully",amount);
    Ok(())
}