use anchor_lang::prelude::*;

#[error_code]
pub enum MangoFarmError {
  #[msg("UnAuthorized Access")]
  UnAuthorizedAccess,

  #[msg("Invariant Vault Balance")]
  InvariantVaultBalance,

  #[msg("Not A Dev Pubkey")]
  NotDevPubKey,

  #[msg("User Already Initialized")]
  UserAlreadyInitialized,

  #[msg("Invalid Mango Farm Pubkey")]
  InvalidMangoFarm,

  #[msg("Invalid PDA Owner Pubkey")]
  InvalidPDAOwner,

  #[msg("Invalid PDA")]
  InvalidPDA,

  #[msg("Self Referral Not Allowed")]
  SelfReferralNotAllowed
}