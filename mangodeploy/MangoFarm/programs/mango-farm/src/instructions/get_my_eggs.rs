use anchor_lang::prelude::*;
use crate::state::{MangoFarm,User};

pub fn get_my_eggs(ctx: Context<GetMyEggs>) -> Result<u64> {
    let user_state = &ctx.accounts.user_pda.user_state;
    let has_eggs = user_state.get_my_eggs(MangoFarm::EGGS_TO_HATCH_1MINERS);
    Ok(has_eggs)
}

#[derive(Accounts)]
pub struct GetMyEggs<'info> {
   // The user pda
   pub user_pda: Account<'info,User>
}



