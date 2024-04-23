use anchor_lang::prelude::*;

#[account]
pub struct Reward {
    pub user: Pubkey,        // 32
    pub repo_pda: Pubkey,    // 32
    pub last_claim: u128,    // 16
    pub total_claimed: u128, // 16
    pub bump: u8,            // 1
}

impl Reward {
    pub const MAX_SIZE: usize = 8 + 32 + 32 + 16 + 16 + 1;
}
