use anchor_lang::prelude::*;
use crate::state::{SolanaFarm,User};
use crate::errors::SolanaFarmError;
use crate::utils::{transfer_from_vault, transfer_to_vault};
use crate::constants::{DEV_PUB_KEY, PROGRAM_PUB_KEY, SOLANA_FARM_PUB_KEY};

pub fn buy_eggs(ctx: Context<BuyEggs>,sol_value: u64) -> Result<()> {
    
    // derive accounts
    let solana_farm = &mut ctx.accounts.solana_farm;
    let user_pda = &mut ctx.accounts.user_pda;
    
    let pda = Pubkey::find_program_address(&[b"solana_farm_user_pda".as_ref(),ctx.accounts.user.key().as_ref()], &PROGRAM_PUB_KEY);
    require_keys_eq!(user_pda.key(),pda.0,SolanaFarmError::InvalidPDAOwner);

    // buy eggs
    let contract_balance: u64 = ctx.accounts.solana_farm_vault.get_lamports();
    let ref_pda = &mut ctx.accounts.ref_pda;
    let devfee = solana_farm.buy_eggs(
        sol_value, 
        contract_balance, 
        user_pda
    );

    // hatch eggs
    solana_farm.hatch_eggs(user_pda,ref_pda)?;

    // transfer
    transfer_to_vault(&ctx,sol_value)?;
    
    // transfer dev fee
    transfer_from_vault(
        &ctx.accounts.system_program, 
        &ctx.accounts.solana_farm_vault, 
        &ctx.accounts.dev, 
        devfee, 
        ctx.bumps.solana_farm_vault
    )
}

#[derive(Accounts)]
pub struct BuyEggs<'info> {   
    // The data account of solana farm
    #[account(mut, address = SOLANA_FARM_PUB_KEY @ SolanaFarmError::InvalidSolanaFarm)] 
    solana_farm: Account<'info,SolanaFarm>,
    
    // The system account of solana farm vault
    #[account(mut,seeds = [b"solana_farm_vault".as_ref()], bump)]
    pub solana_farm_vault: SystemAccount<'info>,

    // The user pda  
    #[account(mut)]
    pub user_pda: Account<'info,User>,
    
    // The referral pda
    #[account(mut,owner = PROGRAM_PUB_KEY @ SolanaFarmError::InvalidPDA)]
    pub ref_pda: Option<Account<'info,User>>,

    // The signer
    #[account(mut, signer)]
    pub user: Signer<'info>,
    
    // dev system account
    #[account(mut, address = DEV_PUB_KEY @ SolanaFarmError::NotDevPubKey)]
    pub dev: SystemAccount<'info>,
    
    // system account
    pub system_program: Program<'info, System>
}