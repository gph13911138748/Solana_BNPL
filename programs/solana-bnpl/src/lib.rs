use anchor_lang::prelude::*;

mod state;
mod instructions;
mod utils;

use instructions::*;


declare_id!("66WHVPBSmijsKqa2ATUjC5LTFGbPfWrpeG3bARKexiVZ");

#[program]
pub mod solana_bnpl {
    use super::*;

    pub fn airdrop_10(ctx: Context<Airdrop>) -> Result<()> {
        airdrop(ctx)
    }

    //initialize mint account for using this tokens for airdrop and rewards
    pub fn initialize_mint_account(ctx: Context<InitializeMintAccount>) -> Result<()> {
        initialize_mint(ctx)
    }

    pub fn initialize_authority(ctx: Context<InitializeAuth>) -> Result<()> {
        initialize_auth(ctx)
    }

    //初始化vault,记录timestamps等信息
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        ctx.accounts.vault.token_account = ctx.accounts.token_account.key();
        ctx.accounts.vault.authority = ctx.accounts.wallet.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
