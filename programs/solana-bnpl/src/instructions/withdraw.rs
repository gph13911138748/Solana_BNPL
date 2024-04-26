use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, MintTo};
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use crate::{state::*, StakeError};
use crate::utils::*;

#[derive(Accounts)]
pub struct Withdraw<'info> { //assume that withdraw all the money
    #[account(mut)]
    pub wallet: Signer<'info>,
    #[account(
        seeds = [
            b"vault",
        ],
        bump,
        mut,
        has_one = mint,
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        seeds = [b"mint"],
        bump,
        mut,
    )]
    pub mint: Account<'info,Mint>,
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
    pub system_program: Program<'info,System>,
}

impl<'info> Withdraw<'info> {
    pub fn mint_token_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.mint.to_account_info(),
        };
    
        let cpi_program = self.token_program.to_account_info();

        CpiContext::new(
            cpi_program, 
            cpi_accounts,
        )
    }
}

pub fn withdraw_(ctx: Context<Withdraw>) -> Result<()> {
    //calculate the reward and send
    let staked_at = ctx.accounts.authority.stake_at.unwrap();
    let minimum_stake_period = 100; //assume 100 seconds for minimum stake period 
    let reward_emission = 1; //assume reward 1 token per seconds

    let (reward_tokens, _current_time, is_eligible_for_reward) = calc_reward(
        staked_at, 
        minimum_stake_period, 
        reward_emission,
    ).unwrap();

    let seeds = &[b"mint".as_ref(),&[ctx.bumps.mint]];
    let signer = [&seeds[..]];
 
    if is_eligible_for_reward {
        mint_to(
            ctx.accounts.mint_token_ctx().with_signer(&signer),
             reward_tokens
        )?;
    } else {
        return err!(StakeError::IneligibleForReward);
    }

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.token_account_pool.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint.to_account_info(),
        },
        &signer,
    );

    token::transfer(cpi_ctx, ctx.accounts.authority.amount)?;
    msg!("withdraw {} tokens successfully", ctx.accounts.authority.amount);

    let authority = &mut ctx.accounts.authority;
    authority.amount = 0;
    authority.stake_at = None;

    Ok(())
}