use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VoteType {
    Up,
    Down,
}

#[account]
pub struct Vote {
    pub voter: Pubkey,       // 32
    pub repo_pda: Pubkey,    // 32
    pub vote_type: VoteType, // 3
}

impl Vote {
    pub const MAX_SIZE: usize = 8 + 32 + 32 + 3;
}