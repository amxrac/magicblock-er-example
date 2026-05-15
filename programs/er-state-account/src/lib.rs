#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::ephemeral;

mod errors;
mod instructions;
mod state;

use instructions::*;

declare_id!("9hG187VazKdEZcYbsEcoPuPEWwkfF9HccUDTAJzuEcg3");

#[ephemeral]
#[program]
pub mod er_state_account {

    use super::*;

    pub fn initialize(ctx: Context<InitUser>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;

        Ok(())
    }

    pub fn update(ctx: Context<UpdateUser>, new_data: u64) -> Result<()> {
        ctx.accounts.update(new_data)?;

        Ok(())
    }

    pub fn update_commit(ctx: Context<UpdateCommit>, new_data: u64) -> Result<()> {
        ctx.accounts.update_commit(new_data)?;

        Ok(())
    }

    pub fn delegate(ctx: Context<Delegate>) -> Result<()> {
        ctx.accounts.delegate()?;

        Ok(())
    }

    pub fn undelegate(ctx: Context<Undelegate>) -> Result<()> {
        ctx.accounts.undelegate()?;

        Ok(())
    }

    pub fn close(ctx: Context<CloseUser>) -> Result<()> {
        ctx.accounts.close()?;

        Ok(())
    }

    // vrf instructions
    // vrf instruction outside ephemeral rollup
    pub fn roll_data(ctx: Context<RollData>, client_seed: u8) -> Result<()> {
        ctx.accounts.roll_data(client_seed)?;
        Ok(())
    }

    pub fn consume_roll_data(ctx: Context<ConsumeRollData>, randomness: [u8; 32]) -> Result<()> {
        ctx.accounts.consume_roll_data(randomness)?;
        Ok(())
    }

    // vrf instruction inside ephemeral rollup
    pub fn roll_data_er(ctx: Context<RollDataER>, client_seed: u8) -> Result<()> {
        ctx.accounts.roll_data_er(client_seed)?;
        Ok(())
    }

    pub fn consume_roll_data_er(
        ctx: Context<ConsumeRollDataEr>,
        randomness: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.consume_roll_data_er(randomness)?;
        Ok(())
    }
}
