use anchor_lang::prelude::*;
use constants::PROGRAM_PUB_KEY;
use instructions::*;

pub mod state;
pub mod errors;
pub mod instructions;
pub mod utils;
pub mod constants;


declare_id!(PROGRAM_PUB_KEY);

#[program]
pub mod mango_farm {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
       instructions::initialize_user(ctx) 
    } 

    pub fn hatch_eggs(ctx: Context<HatchEggs>) -> Result<()> {
        instructions::hatch_eggs(ctx)
    }

    pub fn buy_eggs(ctx: Context<BuyEggs>, sol_value: u64) -> Result<()> {
        instructions::buy_eggs(ctx, sol_value)
    }

    pub fn sell_eggs(ctx: Context<SellEggs>) -> Result<()> {
        instructions::sell_eggs(ctx)
    }

    pub fn get_accumulated_sol(ctx: Context<GetAccmulatedSol>) -> Result<u64> {
        instructions::get_accumulated_sol(ctx)
    }

    pub fn get_my_eggs(ctx: Context<GetMyEggs>) -> Result<u64> {
        instructions::get_my_eggs(ctx)
    }

    pub fn recover_sol(ctx: Context<RecoverSol>) -> Result<()> {
        instructions::recover_sol(ctx)
    }

}