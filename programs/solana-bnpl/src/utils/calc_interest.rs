use anchor_lang::prelude::*;
use crate::StakeError;

pub fn calc_interest(
    borrow_at: i64,
    emission: u64,//Define the number of reward tokens that can be issued
) -> Result<(u64, i64)> {
    let clock = Clock::get().unwrap();
    let current_time = clock.unix_timestamp;

    let time_i64 = current_time.checked_sub(borrow_at).ok_or(StakeError::ProgramSubError)?;

    let time_u64 = match u64::try_from(time_i64) {
        Ok(time) => time,
        _ => return err!(StakeError::FailedTimeConversion)
    };

    let tokens = time_u64.checked_mul(emission).ok_or(StakeError::ProgramMulError)?;
    Ok((tokens, current_time))
}