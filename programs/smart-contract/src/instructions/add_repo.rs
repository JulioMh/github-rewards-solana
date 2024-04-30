use crate::state::{Repo,RepoPayload};
use anchor_lang::prelude::*;

pub fn add_repo(
        ctx: Context<AddRepo>,
        payload: RepoPayload,
    )-> Result<()> {
        let publisher = ctx.accounts.publisher.key();
        let repo = &mut ctx.accounts.repo;
        repo.owner = payload.owner;
        repo.name = payload.name;
        repo.branch = payload.branch;
        repo.votes = 0;
        repo.publisher = publisher;
        repo.bump = ctx.bumps.repo;
        repo.total_claimed = 0;

        Ok(())
}

#[derive(Accounts)]
#[instruction(payload: RepoPayload)]
pub struct AddRepo<'info> {
    #[account(
        init,
        seeds = [b"repo", payload.owner.as_bytes(), payload.name.as_bytes(), payload.branch.as_bytes()],
        bump,
        payer = publisher,
        space = Repo::size(&payload.name, &payload.owner, &payload.branch) 
    )]
    pub repo: Account<'info, Repo>,
    #[account(mut)]
    pub publisher: Signer<'info>,
    pub system_program: Program<'info, System>,
}
