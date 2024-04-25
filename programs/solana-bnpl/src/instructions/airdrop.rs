use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{self, mint_to, Mint, MintTo, Token, TokenAccount}};

#[derive(Accounts)]
pub struct Airdrop<'info> {
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
        init_if_needed,
        payer = wallet,
        associated_token::mint = mint,
        associated_token::authority = wallet,
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

pub fn airdrop(ctx: Context<Airdrop>) -> Result<()> {
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
        10)?;
    Ok(())
}