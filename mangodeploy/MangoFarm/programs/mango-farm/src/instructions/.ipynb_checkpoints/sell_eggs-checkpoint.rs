use anchor_lang::prelude::*;
use crate::state::{SolanaFarm,User};
use crate::errors::SolanaFarmError;
use crate::utils::transfer_from_vault;
use crate::constants::{SOLANA_FARM_PUB_KEY,DEV_PUB_KEY,PROGRAM_PUB_KEY};

pub fn sell_eggs(ctx: Context<SellEggs>) -> Result<()> {
    
    let solana_farm = &mut ctx.accounts.solana_farm;
    let user_pda = &mut ctx.accounts.user_pda;
    let system_program = &mut ctx.accounts.system_program;
    let solana_farm_vault = &mut ctx.accounts.solana_farm_vault;
    let dev = &ctx.accounts.dev;
    let bumps = ctx.bumps.solana_farm_vault;

    let pda = Pubkey::find_program_address(&[b"solana_farm_user_pda".as_ref(),ctx.accounts.user.key().as_ref()], &PROGRAM_PUB_KEY);
    require_keys_eq!(user_pda.key(),pda.0,SolanaFarmError::InvalidPDAOwner);

    // sell eggs
    let (dev_fee, amount_to_transfer) = solana_farm.sell_eggs(solana_farm_vault.get_lamports(),user_pda);

    // transfer dev fee
    transfer_from_vault(
        system_program,
        solana_farm_vault,
        dev,
        dev_fee,
        bumps
    )?;
    
    // transfer left amount to user
    transfer_from_vault(
      system_program,
      solana_farm_vault,
      &ctx.accounts.user,
      amount_to_transfer,
      bumps  
    )
}

#[derive(Accounts)]
pub struct SellEggs<'info> {
    
    // The data account of solana farm
    #[account(mut, address = SOLANA_FARM_PUB_KEY @ SolanaFarmError::InvalidSolanaFarm)] 
    pub solana_farm: Account<'info,SolanaFarm>,
    
    // The solana farm vault
    #[account(
        mut,
        seeds = [b"solana_farm_vault".as_ref()],
        bump
    )]
    pub solana_farm_vault: SystemAccount<'info>,

    // The user pda
    #[account(mut)]
    pub user_pda: Account<'info,User>,

    // The signer
    #[account(mut,signer)]
    pub user: Signer<'info>,
    
    // The dev
    #[account(mut, address = DEV_PUB_KEY @ SolanaFarmError::NotDevPubKey)]
    pub dev: SystemAccount<'info>,

    // The system program
    pub system_program: Program<'info, System> 
}