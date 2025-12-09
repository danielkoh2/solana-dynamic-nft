use anchor_lang::prelude::*;

use crate::error::MetaplexCoreError;

#[account]
#[derive(InitSpace)]
pub struct WhitelistedCreators {
    pub creators: [Pubkey; 10],
    pub num_creators: u8,
    pub bump: u8,
}

impl WhitelistedCreators {
    pub fn contains(&self, creator: &AccountInfo) -> bool {
        self.creators[..self.num_creators as usize].contains(creator.key)
    }

    pub fn whitelist_creator(&mut self, creator: &AccountInfo)->Result<()>{
        // Check if array is full
        if self.num_creators as usize >= self.creators.len() {
            return err!(MetaplexCoreError::CreatorListFull);
        }

        // Check if already whitelisted
        if self.contains(creator){
            return err!(MetaplexCoreError::CreatorAlreadyWhitelisted);
        }

        // Add the creator at the current num_creators index
        self.creators[self.num_creators as usize] = creator.key();
        self.num_creators += 1;
        Ok(())
    }
}