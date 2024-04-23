use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("User has already voted")]
    VotedAlready,
}
