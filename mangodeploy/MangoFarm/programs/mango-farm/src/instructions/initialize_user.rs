use anchor_lang::prelude::*;
use crate::errors::MangoFarmError;
use crate::state::{User,UserState};


pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
  let user_state = &mut ctx.accounts.user_pda.user_state;
  require!(user_state.is_initialized == false,MangoFarmError::UserAlreadyInitialized);
  user_state.initialize()
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    // The program derived address of user
    #[account(
        init,
        payer = user,
        space = UserState::MAX_SIZE,
        seeds = [b"mango_farm_user_pda".as_ref(), user_account.key().as_ref()],
        bump
    )]
    pub user_pda: Account<'info,User>,
    
    // user signer   
    #[account(mut,signer)]
    pub user: Signer<'info>,

    /// CHECK:: only used as a signing PDA
    pub user_account: UncheckedAccount<'info>,
    
    // system account
    pub system_program: Program<'info, System>  
}
