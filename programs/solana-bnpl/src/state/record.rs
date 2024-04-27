use anchor_lang::prelude::*;

#[account]
pub struct Record { // 8
    pub amount: u64, // 8
    pub borrow_at: Option<i64>, // 1 + 8
    pub borrow_times: u64, // 8
    pub token_account: Pubkey, //32
    pub wallet: Pubkey, //32
}

impl Record {
    pub fn init(&mut self, amount: u64) {
        self.amount = amount;

        let clock = Clock::get().unwrap();
        let borrow_at = clock.unix_timestamp;
        self.borrow_at = Some(borrow_at);
        self.borrow_times += 1;
    }
}