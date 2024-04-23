use anchor_lang::prelude::*;

use crate::state::*;
use instructions::*;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("2F3RA1FWPnsWjmhqjwSLb5pjsPKRc8QaiUaQ5cLWduhM");

#[program]
pub mod smart_contract {

    use super::*;

    pub fn add_repo(ctx: Context<AddRepo>, payload: RepoPayload) -> Result<()> {
        instructions::add_repo::add_repo(ctx, payload)
    }

    pub fn vote_repo(ctx: Context<VoteRepo>, payload: VoteRepoPayload) -> Result<()> {
        instructions::vote_repo::vote_repo(ctx, payload)
    }

    pub fn init_token(ctx: Context<InitToken>) -> Result<()> {
        instructions::init_token::init_token(ctx)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>, payload: ClaimRewardsPayload) -> Result<()> {
        instructions::claim_rewards::claim_rewards(ctx, payload)
    }
}
