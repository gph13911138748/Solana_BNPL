use anchor_lang::prelude::*;

mod state;
mod instructions;
mod utils;

use instructions::*;


declare_id!("66WHVPBSmijsKqa2ATUjC5LTFGbPfWrpeG3bARKexiVZ");

#[program]
pub mod solana_bnpl {
    use super::*;

    pub fn airdrop(ctx: Context<Airdrop>) -> Result<()> {
        airdrop_10(ctx)
    }

    //initialize mint account for using this tokens for airdrop and rewards
    pub fn initialize_mint_account(ctx: Context<InitializeMintAccount>) -> Result<()> {
        initialize_mint(ctx)
    }

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        initialize_vault_(ctx)
    }

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        initialize_pool_(ctx)
    }

    pub fn initialize_auth(ctx: Context<InitializeAuth>) -> Result<()> {
        initialize_auth_(ctx)
    }

    pub fn stake(ctx: Context<Stake>,amount: u64) -> Result<()> {
        stake_(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw_(ctx)
    }
}

#[error_code]
pub enum StakeError {
    #[msg("unable to get stake details bump")]
    StakeBumpError,
    #[msg("unable to get token authority bump")]
    TokenAuthBumpError,
    #[msg("the minimum staking period in secs can't be negative")]
    NegativePeriodValue,
    #[msg("the given token account has no token")]
    TokenAccountEmpty,
    #[msg("the collection field in the metadata is not verified")]
    CollectionNotVerified,
    #[msg("the collection doesn't match the staking details")]
    InvalidCollection,
    #[msg("the minimum stake period for the rewards not completed yet")]
    IneligibleForReward,
    #[msg("the staking is not currently active")]
    StakingInactive,
    #[msg("failed to convert the time to u64")]
    FailedTimeConversion,
    #[msg("unable to add the given values")]
    ProgramAddError,
    #[msg("unable to subtract the given values")]
    ProgramSubError,
    #[msg("unable to multiply the given values")]
    ProgramMulError,
}