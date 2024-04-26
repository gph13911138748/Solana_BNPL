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

    //初始化vault,记录timestamps等信息
    pub fn initialize_vault_(ctx: Context<InitializeVault>) -> Result<()> {
        initialize_vault(ctx)
    }

    pub fn initialize_pool_(ctx: Context<InitializePool>) -> Result<()> {
        initialize_pool(ctx)
    }

    pub fn initialize_authority(ctx: Context<InitializeAuth>) -> Result<()> {
        initialize_auth(ctx)
    }

    pub fn stake_(ctx: Context<Stake>,amount: u64) -> Result<()> {
        stake(ctx, amount)
    }

    pub fn secure_withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw(ctx)
    }
}

#[error_code]
pub enum StakeError {
    #[msg("unable to get stake details bump")]
    StakeBumpError,
    #[msg("unable to get token authority bump")]
    TokenAuthBumpError,
    #[msg("unable to get token authority bump")]
    NftAuthBumpError,
    #[msg("unable to get nft record bump")]
    NftBumpError,
    #[msg("the minimum staking period in secs can't be negative")]
    NegativePeriodValue,
    #[msg("the given mint account doesn't belong to NFT")]
    TokenNotNFT,
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