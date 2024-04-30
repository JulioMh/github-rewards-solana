use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use solana_program::pubkey;

use crate::{
    state::{Repo, RepoPayload, Reward},
    utils::CustomError,
};

pub fn claim_rewards(ctx: Context<ClaimRewards>, payload: ClaimRewardsPayload) -> Result<()> {
    let seed = b"token";
    let bump = ctx.bumps.token;
    let signer: &[&[&[u8]]] = &[&[seed, &[bump]]];

    let just_initialized =
        ctx.accounts.reward.user.key() == pubkey!("11111111111111111111111111111111");

    require!(
        just_initialized || payload.timestamp < ctx.accounts.reward.last_claim,
        CustomError::ClaimedAlready
    );

    if just_initialized {
        ctx.accounts.reward.initialize(
            ctx.accounts.signer.key(),
            ctx.accounts.repo.key(),
            payload.timestamp,
            ctx.bumps.reward,
        );
    }

    ctx.accounts
        .reward
        .update_total_claimed(payload.commits.into());

    ctx.accounts
        .repo
        .update_total_claimed(payload.commits.into());

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.token.to_account_info(),
                mint: ctx.accounts.token.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
            },
            signer,
        ),
        payload.commits.checked_mul(1000000000).unwrap(),
    )?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ClaimRewardsPayload {
    pub repo: RepoPayload,
    pub commits: u64,
    pub timestamp: u128,
}

#[derive(Accounts)]
#[instruction(payload: ClaimRewardsPayload)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"repo", payload.repo.owner.as_bytes(), payload.repo.name.as_bytes(), payload.repo.branch.as_bytes()],
        bump,
    )]
    pub repo: Account<'info, Repo>,
    #[account(
        init_if_needed,
        seeds = [b"reward", signer.key().as_ref() ,repo.key().as_ref()],
        bump,
        payer=signer,
        space = Reward::MAX_SIZE
    )]
    pub reward: Account<'info, Reward>,
    #[account(
      mut,
      seeds=[b"token"],
      bump,
      mint::authority=token
    )]
    pub token: Account<'info, Mint>,
    #[account(
      init_if_needed,
      payer=signer,
      associated_token::mint = token,
      associated_token::authority = signer,
    )]
    pub destination: Account<'info, TokenAccount>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
