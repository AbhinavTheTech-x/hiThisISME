use anchor_lang::prelude::*;
use crate::state::{MangoFarm,User};
use crate::errors::MangoFarmError;
use crate::utils::{transfer_from_vault, transfer_to_vault};
use crate::constants::{DEV_PUB_KEY, PROGRAM_PUB_KEY, MANGO_FARM_PUB_KEY};

pub fn buy_eggs(ctx: Context<BuyEggs>,sol_value: u64) -> Result<()> {
    
    // derive accounts
    let mango_farm = &mut ctx.accounts.mango_farm;
    let user_pda = &mut ctx.accounts.user_pda;
    
    let pda = Pubkey::find_program_address(&[b"mango_farm_user_pda".as_ref(),ctx.accounts.user.key().as_ref()], &PROGRAM_PUB_KEY);
    require_keys_eq!(user_pda.key(),pda.0,MangoFarmError::InvalidPDAOwner);

    // buy eggs
    let contract_balance: u64 = ctx.accounts.mango_farm_vault.get_lamports();
    let ref_pda = &mut ctx.accounts.ref_pda;
    let devfee = mango_farm.buy_eggs(
        sol_value, 
        contract_balance, 
        user_pda
    );

    // hatch eggs
    mango_farm.hatch_eggs(user_pda,ref_pda)?;

    // transfer
    transfer_to_vault(&ctx,sol_value)?;
    
    // transfer dev fee
    transfer_from_vault(
        &ctx.accounts.system_program, 
        &ctx.accounts.mango_farm_vault, 
        &ctx.accounts.dev, 
        devfee, 
        ctx.bumps.mango_farm_vault
    )
}

#[derive(Accounts)]
pub struct BuyEggs<'info> {   
    // The data account of solana farm
    #[account(mut, address = MANGO_FARM_PUB_KEY @ MangoFarmError::InvalidMangoFarm)] 
    mango_farm: Account<'info,MangoFarm>,
    
    // The system account of solana farm vault
    #[account(mut,seeds = [b"mango_farm_vault".as_ref()], bump)]
    pub mango_farm_vault: SystemAccount<'info>,

    // The user pda  
    #[account(mut)]
    pub user_pda: Account<'info,User>,
    
    // The referral pda
    #[account(mut,owner = PROGRAM_PUB_KEY @ MangoFarmError::InvalidPDA)]
    pub ref_pda: Option<Account<'info,User>>,

    // The signer
    #[account(mut, signer)]
    pub user: Signer<'info>,
    
    // dev system account
    #[account(mut, address = DEV_PUB_KEY @ MangoFarmError::NotDevPubKey)]
    pub dev: SystemAccount<'info>,
    
    // system account
    pub system_program: Program<'info, System>
}