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
    pub fn initialize_vault_(ctx: Context<InitializeVault>) -> Result<()> {
        initialize_vault(ctx)
    }

    pub fn initialize_pool_(ctx: Context<InitializePool>) -> Result<()> {
        initialize_pool(ctx)
    }
}

