use anchor_lang::prelude::*;
use instructions::*;

use crate::state::VoteType;
pub mod instructions;
pub mod state;

declare_id!("2F3RA1FWPnsWjmhqjwSLb5pjsPKRc8QaiUaQ5cLWduhM");

#[program]
pub mod smart_contract {

    use super::*;

    pub fn add_repo(
        ctx: Context<AddRepo>,
        owner: String,
        name: String,
        branch: String,
    ) -> Result<()> {
        instructions::add_repo::add_repo(ctx, owner, name, branch)
    }

    pub fn vote_repo(
        ctx: Context<VoteRepo>,
        owner: String,
        name: String,
        branch: String,
        vote_type: VoteType,
    ) -> Result<()> {
        instructions::vote_repo::vote_repo(ctx, owner, name, branch, vote_type)
    }
}

#[error_code]
pub enum CustomError {
    #[msg("User has already voted")]
    VotedAlready,
}
