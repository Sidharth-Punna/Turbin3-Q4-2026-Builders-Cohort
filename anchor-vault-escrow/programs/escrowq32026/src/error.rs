use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The escrow has expired")]
    EscrowExpired,
    #[msg("The expiration must be in the future")]
    ExpirationInPast,
    #[msg("The escrow can only be refunded once it has expired")]
    EscrowNotExpired,
}
