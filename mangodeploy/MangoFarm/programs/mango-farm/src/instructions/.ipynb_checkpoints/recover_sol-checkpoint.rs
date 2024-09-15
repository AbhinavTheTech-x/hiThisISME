use anchor_lang::prelude::*;
use crate::utils::transfer_from_vault;
use crate::errors::SolanaFarmError;
use crate::constants::OWNER_PUB_KEY;


pub fn recover_sol(ctx: Context<RecoverSol>) -> Result<()> {
   let lamports = ctx.accounts.solana_farm_vault.get_lamports(); 
   let bumps = ctx.bumps.solana_farm_vault;

   // recover sol balance safely
   transfer_from_vault(
    &ctx.accounts.system_program, 
    &ctx.accounts.solana_farm_vault, 
    &ctx.accounts.owner, 
    lamports, 
    bumps
   ) 
}

#[derive(Accounts)]
pub struct RecoverSol<'info> {
  // The system account of solana farm vault
  #[account(mut, seeds = [b"solana_farm_vault".as_ref()],bump)]
  solana_farm_vault: SystemAccount<'info>,

  // owner   
  #[account(mut,signer, address = OWNER_PUB_KEY @ SolanaFarmError::UnAuthorizedAccess)]
  owner: Signer<'info>,

  // system program  
  system_program: Program<'info, System>   
}

