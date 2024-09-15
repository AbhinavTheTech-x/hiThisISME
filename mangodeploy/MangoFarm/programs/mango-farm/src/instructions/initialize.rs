use anchor_lang::prelude::*;
use crate::state::MangoFarm;
use crate::errors::MangoFarmError;
use crate::constants::{MANGO_FARM_PUB_KEY,OWNER_PUB_KEY};

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let mango_farm = &mut ctx.accounts.mango_farm;
    mango_farm.seed_market()
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // The data account of mango farm
    #[account(init,signer,payer = owner, space = MangoFarm::MAX_SIZE, address = MANGO_FARM_PUB_KEY @ MangoFarmError::InvalidMangoFarm)]
    pub mango_farm: Account<'info, MangoFarm>,

    // The owner
    #[account(mut, signer, address = OWNER_PUB_KEY @ MangoFarmError::UnAuthorizedAccess)]
    pub owner: Signer<'info>, 

    // The system program
    pub system_program: Program<'info, System>
}
