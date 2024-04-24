use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

mod state;
mod instructions;
mod utils;

use instructions::*;


declare_id!("66WHVPBSmijsKqa2ATUjC5LTFGbPfWrpeG3bARKexiVZ");

#[program]
pub mod solana_bnpl {
    use super::*;

    //初始化vault,记录timestamps等信息
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        ctx.accounts.vault.token_account = ctx.accounts.token_account.key();
        ctx.accounts.vault.authority = ctx.accounts.wallet.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
