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

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Poll::INIT_SPACE, // ✅ FIXED: Now properly defined
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Poll {
    pub id: u64,
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}

impl Poll {
    /// ✅ **Added `INIT_SPACE` calculation**
    pub const INIT_SPACE: usize = 8 +  // Anchor's account discriminator
        std::mem::size_of::<u64>() + // id
        4 + 256 + // description (4-byte prefix for string length + 256 bytes for content)
        std::mem::size_of::<u64>() + // poll_start
        std::mem::size_of::<u64>() + // poll_end
        std::mem::size_of::<u64>(); // candidate_amount
}
