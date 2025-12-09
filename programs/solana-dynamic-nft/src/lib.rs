use anchor_lang::prelude::*;

declare_id!("G7bEwsmbombKZHHHpqBrSi7prLALAc3xanGseut54TPV");

#[program]
pub mod solana_dynamic_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
