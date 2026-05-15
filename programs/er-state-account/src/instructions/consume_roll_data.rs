use crate::errors::AppError;
use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::rnd::random_u8_with_range;

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct ConsumeRollData<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    /// Signer PDA of the VRF program — proves the oracle produced this callback
    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,
}

impl<'info> ConsumeRollData<'info> {
    pub fn consume_roll_data(&mut self, randomness: [u8; 32]) -> Result<()> {
        // convert VRF randomness to a number between 1–60
        let data = random_u8_with_range(&randomness, 1, 60) as u64;

        let new_data = self
            .user_account
            .data
            .checked_add(data)
            .ok_or(AppError::Overflow)?;

        self.user_account.data = new_data;

        Ok(())
    }
}
