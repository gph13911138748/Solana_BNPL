use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub token_account: Pubkey,
    pub authority: Pubkey,
    pub stake_at: Option<i64>,
}

impl Vault {
    pub fn init(
        token_account: Pubkey,
        authority: Pubkey,
    ) -> Self {
        Self {
            token_account,
            authority,
            stake_at:None,
        }
    }
}