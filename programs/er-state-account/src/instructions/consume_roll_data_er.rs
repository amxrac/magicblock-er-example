use crate::errors::AppError;
use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::{anchor::commit, ephem::commit_accounts};
use ephemeral_vrf_sdk::rnd::random_u8_with_range;

use crate::state::UserAccount;

#[commit]
#[derive(Accounts)]
pub struct ConsumeRollDataEr<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: only needed for PDA seed derivation
    pub user: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    /// Signer PDA of the VRF program — proves the oracle produced this callback
    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,
}

impl<'info> ConsumeRollDataEr<'info> {
    pub fn consume_roll_data_er(&mut self, randomness: [u8; 32]) -> Result<()> {
        let data = random_u8_with_range(&randomness, 1, 60) as u64;

        let new_data = self
            .user_account
            .data
            .checked_add(data)
            .ok_or(AppError::Overflow)?;

        self.user_account.data = new_data;

        commit_accounts(
            &self.payer.to_account_info(),
            vec![&self.user_account.to_account_info()],
            &self.magic_context,
            &self.magic_program,
        )?;

        Ok(())
    }
}
