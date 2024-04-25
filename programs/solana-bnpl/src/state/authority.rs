use anchor_lang::prelude::*;

#[account]
pub struct Authority { //records for a user, owned by the program
    pub token_account: Pubkey,
    pub authority: Pubkey,
    pub amount: i64, // how much money put in the pool
    pub stake_at: Option<i64>,
}