use anchor_lang::prelude::*;

#[account]
pub struct Vault { //program's vault pool
    pub token_account: Pubkey,
    pub authority: Pubkey,
}

impl Vault {
    pub fn init(
        token_account: Pubkey,
        authority: Pubkey,
    ) -> Self {
        Self {
            token_account,
            authority,
        }
    }
}