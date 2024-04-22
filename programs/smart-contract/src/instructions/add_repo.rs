use crate::state::Repo;
use anchor_lang::prelude::*;

pub fn add_repo(
        ctx: Context<AddRepo>,
        owner: String,
        name: String,
        branch: String,
    )-> Result<()> {
        let publisher = ctx.accounts.publisher.key();
        let repo = &mut ctx.accounts.repo;
        repo.owner = owner;
        repo.name = name;
        repo.branch = branch;
        repo.votes = 0;
        repo.publisher = publisher;

        Ok(())
}

#[derive(Accounts)]
#[instruction(owner: String, name: String, branch: String)]
pub struct AddRepo<'info> {
    #[account(
        init,
        seeds = [b"repo", owner.as_bytes(), name.as_bytes(), branch.as_bytes()],
        bump,
        payer = publisher,
        space = Repo::size(&name, &owner, &branch) 
    )]
    pub repo: Account<'info, Repo>,
    #[account(mut)]
    pub publisher: Signer<'info>,
    pub system_program: Program<'info, System>,
}
