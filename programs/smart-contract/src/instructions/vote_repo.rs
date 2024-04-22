use std::str::FromStr;

use crate::{
    state::{Repo, Vote, VoteType},
    CustomError,
};
use anchor_lang::prelude::*;

pub fn vote_repo(
    ctx: Context<VoteRepo>,
    _owner: String,
    _name: String,
    _branch: String,
    vote_type: VoteType,
) -> Result<()> {
    let voter = ctx.accounts.voter.key();
    let repo: &mut Account<'_, Repo> = &mut ctx.accounts.repo;
    let vote = &mut ctx.accounts.vote;

    let default_pubkey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
    let just_initialized = vote.voter.key() == default_pubkey;

    require!(
        just_initialized || vote.vote_type != vote_type,
        CustomError::VotedAlready
    );

    if just_initialized {
        repo.vote(&vote_type);
        vote.voter = voter;
        vote.vote_type = vote_type;
        vote.repo_pda = repo.key();
    } else {
        repo.change_vote(&vote_type);
        vote.vote_type = vote_type;
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(owner: String, name: String, branch: String)]
pub struct VoteRepo<'info> {
    #[account(
        mut,
        seeds = [b"repo", owner.as_bytes(), name.as_bytes(), branch.as_bytes()],
        bump,
    )]
    pub repo: Account<'info, Repo>,
    #[account(
        init_if_needed,
        seeds = [b"vote", repo.key().as_ref(), voter.key().as_ref()],
        bump,
        payer=voter,
        space = Vote::MAX_SIZE
    )]
    pub vote: Account<'info, Vote>,
    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}
