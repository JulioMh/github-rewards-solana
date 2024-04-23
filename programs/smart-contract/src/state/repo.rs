use anchor_lang::prelude::*;

use super::VoteType;

#[account]
pub struct Repo {
    pub bump: u8,                 // 1
    pub approved_timestamp: u128, // 16
    pub votes: i128,              // 16
    pub publisher: Pubkey,        // 32
    pub name: String,             // 4 + len()
    pub owner: String,            // 4 + len()
    pub branch: String,           // 4 + len()
}

impl Repo {
    pub fn size(name: &String, owner: &String, branch: &String) -> usize {
        8 + 32 + 4 + name.len() + 4 + owner.len() + 16 + 4 + branch.len() + 16 + 1
    }

    pub fn vote(&mut self, vote_type: &VoteType) {
        match vote_type {
            VoteType::Up => self.votes = self.votes.checked_add(1).unwrap(),
            VoteType::Down => self.votes = self.votes.checked_sub(1).unwrap(),
        }
    }

    pub fn change_vote(&mut self, vote_type: &VoteType) {
        match vote_type {
            VoteType::Up => self.votes = self.votes.checked_add(2).unwrap(),
            VoteType::Down => self.votes = self.votes.checked_sub(2).unwrap(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct RepoPayload {
    pub owner: String,
    pub name: String,
    pub branch: String,
}
