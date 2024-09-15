use anchor_lang::prelude::*;
use crate::state::{SolanaFarm,User};
use crate::errors::SolanaFarmError;
use crate::constants::{SOLANA_FARM_PUB_KEY,PROGRAM_PUB_KEY};

pub fn hatch_eggs(ctx: Context<HatchEggs>) -> Result<()> {
  
  let solana_farm = &mut ctx.accounts.solana_farm;
  let user_pda = &mut ctx.accounts.user_pda;
  let ref_pda = &mut ctx.accounts.ref_pda;

  let pda = Pubkey::find_program_address(&[b"solana_farm_user_pda".as_ref(),ctx.accounts.user.key().as_ref()], &PROGRAM_PUB_KEY);
  require_keys_eq!(user_pda.key(),pda.0,SolanaFarmError::InvalidPDAOwner);

  solana_farm.hatch_eggs(user_pda, ref_pda)
}

#[derive(Accounts)]
pub struct HatchEggs<'info> {  
  // The account of solana farm
  #[account(mut, address = SOLANA_FARM_PUB_KEY @ SolanaFarmError::InvalidSolanaFarm)] 
  solana_farm: Account<'info,SolanaFarm>,

  // The pda of user
  #[account(mut)]
  pub user_pda: Account<'info,User>,

  // The pda of referral
  #[account(mut,owner = PROGRAM_PUB_KEY @ SolanaFarmError::InvalidPDA)]
  pub ref_pda: Option<Account<'info,User>>,

  // The signer
  #[account(mut, signer)]
  user: Signer<'info>,

  // The system program
  system_program: Program<'info, System>
} 