#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod votingdapp {
    use super::*;

    pub fn initialize_poll(_ctx: Context<InitializePoll>, _poll_id: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(accounts)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 1024,
        seeds = [b"initialize_poll"]
    )]
    pub poll: Account<'info, Poll>,
}

#[account]
pub struct Poll<'info> {
    pub id: u64,
    #[max_len(280)]
    pub description: <String>,
    pub poll_start: <u64>,
    pub poll_end: <u64>,
    pub candidate_amount: <u64>,
}
