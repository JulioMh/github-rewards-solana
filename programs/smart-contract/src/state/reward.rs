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

    pub fn initialize(&mut self, signer: Pubkey, repo: Pubkey, timestamp: u128, bump: u8) {
        self.user = signer;
        self.repo_pda = repo;
        self.bump = bump;
        self.total_claimed = 0;
        self.last_claim = timestamp;
    }

    pub fn update_total_claimed(&mut self, rewards: u128) {
        self.total_claimed = self.total_claimed.checked_add(rewards).unwrap();
    }
}
