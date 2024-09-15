use anchor_lang::prelude::*;
use crate::utils::transfer_from_vault;
use crate::errors::MangoFarmError;
use crate::constants::OWNER_PUB_KEY;


pub fn recover_sol(ctx: Context<RecoverSol>) -> Result<()> {
   let lamports = ctx.accounts.mango_farm_vault.get_lamports(); 
   let bumps = ctx.bumps.mango_farm_vault;

   // recover sol balance safely
   transfer_from_vault(
    &ctx.accounts.system_program, 
    &ctx.accounts.mango_farm_vault, 
    &ctx.accounts.owner, 
    lamports, 
    bumps
   ) 
}

#[derive(Accounts)]
pub struct RecoverSol<'info> {
  // The system account of solana farm vault
  #[account(mut, seeds = [b"mango_farm_vault".as_ref()],bump)]
  mango_farm_vault: SystemAccount<'info>,

  // owner   
  #[account(mut,signer, address = OWNER_PUB_KEY @ MangoFarmError::UnAuthorizedAccess)]
  owner: Signer<'info>,

  // system program  
  system_program: Program<'info, System>   
}

