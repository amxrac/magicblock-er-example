use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::{anchor::commit, ephem::commit_accounts};
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_vrf_sdk::consts::DEFAULT_EPHEMERAL_QUEUE;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;

use crate::instruction::ConsumeRollDataEr;
use crate::state::UserAccount;

#[vrf]
#[derive(Accounts)]
pub struct RollDataER<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,
    /// CHECK: Oracle queue for ephemeral rollup VRF
    #[account(mut, address = DEFAULT_EPHEMERAL_QUEUE)]
    pub oracle_queue: AccountInfo<'info>,
}

impl<'info> RollDataER<'info> {
    pub fn roll_data_er(&self, client_seed: u8) -> Result<()> {
        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: self.user.key(),
            oracle_queue: self.oracle_queue.key(),
            callback_program_id: crate::ID,
            callback_discriminator: ConsumeRollDataEr::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed; 32],
            accounts_metas: Some(vec![
                // writable player_account PDA for the callback
                SerializableAccountMeta {
                    pubkey: self.user_account.key(),
                    is_signer: false,
                    is_writable: true,
                },
            ]),
            ..Default::default()
        });

        self.invoke_signed_vrf(&self.user.to_account_info(), &ix)?;
        Ok(())
    }
}
