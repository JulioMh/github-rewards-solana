use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("User has already voted")]
    VotedAlready,
    #[msg("Commits already claimed")]
    ClaimedAlready,
    #[msg("This coupon is invalid")]
    InvalidCoupon,
}
