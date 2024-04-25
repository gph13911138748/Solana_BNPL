use anchor_lang::prelude::*;

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
    pub system_program: Program<'info,System>,
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
    Ok(())
}