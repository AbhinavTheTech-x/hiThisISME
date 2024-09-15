use anchor_lang::prelude::*;

#[account]
pub struct User {
  pub user_state: UserState  
}

#[derive(Clone,AnchorSerialize, AnchorDeserialize)]
pub struct UserState {
  pub is_initialized: bool, 
  pub hatchery_miners: u64,
  pub claimed_eggs: u64,
  pub last_hatch: u64,
  pub referral: Pubkey,
}

impl UserState {
   
   pub const MAX_SIZE: usize = 1 + 8 + 8 + 8 + 32 + 8; // 8 bytes extra
   
   pub fn initialize(&mut self) -> Result<()> {
      self.is_initialized = true;
      Ok(())
   }

   // helper functions
   pub fn set_hatchery_miners(&mut self, value: u64) {
    self.hatchery_miners += value
   }

   pub fn set_claimed_eggs(&mut self, value: u64) {
    self.claimed_eggs += value
   }

   pub fn set_last_hatch(&mut self, value: u64) {
    self.last_hatch = value
   }

   pub fn set_referral(&mut self, ref_id: Pubkey) {
    self.referral = ref_id
   }

   pub fn get_my_eggs(&self, eggs_to_hatch_1miners:u64) -> u64 {
       self.claimed_eggs + self.get_eggs_since_last_hatch(eggs_to_hatch_1miners)    
   }

    pub fn get_my_miners(&self) -> u64 {
       self.hatchery_miners
    }  

    // utils internal functions
   fn get_eggs_since_last_hatch(&self, eggs_to_hatch_1miners:u64) -> u64 {
      let clock = Clock::get().unwrap();
      let seconds_passed = Self::min(
         eggs_to_hatch_1miners,
         clock.unix_timestamp as u64 - self.last_hatch
      );
      seconds_passed * self.hatchery_miners
   }

    fn min(a: u64,b: u64) -> u64 {
       if a < b {a} else {b} 
    } 
}