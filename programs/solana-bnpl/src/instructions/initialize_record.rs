use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{ Mint, Token, TokenAccount}};

use crate::state::*;

#[derive(Accounts)]
pub struct InitializeRecord<'info> {
    #[account(
        init,
        payer = wallet,
        space = 8 + 32 + 32 + 8 + 1 + 8 + 8,
        seeds = [
            b"record",
            wallet.key().as_ref(),
        ],
        bump,
    )]
    pub record: Account<'info, Record>, //record investors's information
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
    // #[account(
    //     seeds = [
    //         b"vault",
    //     ],
    //     bump,
    //     mut,
    // )]
    // pub vault: Account<'info, Vault>,
}

pub fn  initialize_record_(ctx: Context<InitializeRecord>) -> Result<()> {
    let record = &mut ctx.accounts.record;

    record.token_account = ctx.accounts.pool_token_account.key();
    record.wallet = ctx.accounts.wallet.key();
    record.amount = 0;
    record.borrow_at = None;
    record.borrow_times = 0;

    Ok(())
}