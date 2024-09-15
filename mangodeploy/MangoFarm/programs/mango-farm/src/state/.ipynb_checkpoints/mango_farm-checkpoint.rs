use anchor_lang::prelude::*;
use crate::{errors::MangoFarmError, state::User};

#[account]
pub struct MangoFarm {
    pub initialized: bool,
    pub market_eggs: u64
}

impl MangoFarm {

    const PSN: u64 = 10000;
    const PSNH: u64 = 5000;
    const DEV_FEE_VAL: u64 = 8;
    const REF_DENOMINATOR: u64 = 8; 
    pub const EGGS_TO_HATCH_1MINERS: u64= 1080000;
    pub const MAX_SIZE: usize = 1 + 8 + (8); // 8 bytes extra

    pub fn seed_market(&mut self) -> Result<()> {
        // initialize solana farm
        self.initialized = true;
        self.market_eggs = 108000000000;
        Ok(())
    }


    pub fn hatch_eggs<'info>(&mut self,user:&mut Account<'info,User>, referral: &mut Option<Account<'info, User>>) -> Result<()>  {
        let user_pda = user.key();
        let user_state = &mut user.user_state;
        let eggs_used = user_state.get_my_eggs(Self::EGGS_TO_HATCH_1MINERS);

        if referral.is_some() {           
            let ref_pda = referral.as_mut().unwrap();
            require_keys_neq!(user_pda,ref_pda.key(), MangoFarmError::SelfReferralNotAllowed);
            user_state.set_referral(ref_pda.key());
            ref_pda.user_state.set_claimed_eggs(eggs_used / Self::REF_DENOMINATOR);  
        }

        let new_miners = eggs_used / Self::EGGS_TO_HATCH_1MINERS;
        user_state.set_hatchery_miners(new_miners);
        user_state.claimed_eggs = 0;
        user_state.set_last_hatch(Clock::get()?.unix_timestamp as u64);
        
        // nerf miners
        self.market_eggs += eggs_used / 5;
        Ok(())
    }

    pub fn sell_eggs<'info>(&mut self, contract_balance: u64,user: &mut Account<'info,User>) -> (u64,u64) {
        let user_state = &mut user.user_state;

        let has_eggs = user_state.get_my_eggs(Self::EGGS_TO_HATCH_1MINERS);        
        let egg_value: u64 = self.calculate_egg_sell(has_eggs,contract_balance);
        let fee = self.dev_fee(egg_value);

        // reset user state
        user_state.claimed_eggs = 0;
        let clock = Clock::get().unwrap();
        user_state.set_last_hatch(clock.unix_timestamp as u64);

        self.market_eggs += has_eggs;
        // return fee and value to transfer
        (fee,egg_value-fee)
    }

    pub fn buy_eggs<'info>(&mut self,sol_value: u64, contract_balance: u64, user: &mut Account<'info,User>) -> u64 {
        let user_state = &mut user.user_state;    
        let mut eggs_bought = self.calculate_egg_buy(sol_value,contract_balance);
        eggs_bought = eggs_bought - self.dev_fee(eggs_bought);
        user_state.set_claimed_eggs(eggs_bought); 
        return self.dev_fee(sol_value);
    }

    // read only functions
    pub fn get_accumulated_rewards(&self, has_eggs: u64, contract_balance: u64) -> u64 {
        self.calculate_egg_sell(has_eggs,contract_balance)
    }

    // utils functions
    fn calculate_egg_sell(&self, eggs: u64, contract_balance: u64) -> u64 {
        Self::PSN * contract_balance / (Self::PSNH + (((Self::PSN * self.market_eggs) + (Self::PSNH * eggs)) / eggs))
    }

    fn calculate_egg_buy(&self, sol: u64, contract_balance: u64) -> u64 {
        Self::PSN * self.market_eggs / (Self::PSNH + (((Self::PSN * contract_balance) + (Self::PSNH * sol)) / sol))
    }

    fn dev_fee(&self, amount: u64) -> u64 {
        amount * Self::DEV_FEE_VAL / 100
    }
}
