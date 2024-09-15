use anchor_lang::prelude::*;
use crate::state::{MangoFarm,User};

pub fn get_accumulated_sol(ctx: Context<GetAccmulatedSol>) -> Result<u64> {
    let mango_farm = &ctx.accounts.mango_farm;
    let user_state = &ctx.accounts.user_pda.user_state;

    let contract_balance = ctx.accounts.mango_farm_vault.get_lamports();
    let has_eggs = user_state.get_my_eggs(MangoFarm::EGGS_TO_HATCH_1MINERS);
    let sol_accumulated = mango_farm.get_accumulated_rewards(has_eggs,contract_balance);
    Ok(sol_accumulated)
}

#[derive(Accounts)]
pub struct GetAccmulatedSol<'info> {
   // The solana farm 
   pub mango_farm: Account<'info,MangoFarm>,

   // The solana farm vault
   pub mango_farm_vault: SystemAccount<'info>,

   // The user pda
   pub user_pda: Account<'info,User>
}



