use anchor_lang::prelude::*;

#[account]
pub struct Authority { //records for a user, owned by the program
    pub token_account: Pubkey,
    pub wallet: Pubkey,
    pub amount: u64, // how much money put in the pool
    pub stake_at: Option<i64>,
}

impl Authority {
    pub fn init(&mut self, amount: u64) {
        self.amount = amount;

        let clock = Clock::get().unwrap();
        let stake_time = clock.unix_timestamp;
        self.stake_at = Some(stake_time);
    }
}