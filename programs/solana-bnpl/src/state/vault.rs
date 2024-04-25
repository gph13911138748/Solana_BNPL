use anchor_lang::prelude::*;

#[account]
pub struct Vault { //program's vault pool
    pub token_account: Pubkey,
    pub mint: Pubkey, //(authority)
}

impl Vault {
    pub fn init(
        token_account: Pubkey,
        mint: Pubkey,
    ) -> Self {
        Self {
            token_account,
            mint,
        }
    }
}