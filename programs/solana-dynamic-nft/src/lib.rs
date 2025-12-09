use anchor_lang::prelude::*;
mod error;
mod state;
mod instructions;

use instructions::*;

declare_id!("G7bEwsmbombKZHHHpqBrSi7prLALAc3xanGseut54TPV");

#[program]
pub mod solana_dynamic_nft {
    use super::*;

    pub fn whitelist_creator(ctx: Context<WhitelistCreator>) -> Result<()> {
        ctx.accounts.whitelist_creator()
    }

    pub fn create_collection(ctx: Context<CreateCollection>, args: CreateCollectionArgs) -> Result<()> {
        ctx.accounts.create_collection(args, &ctx.bumps)
    }

    pub fn mint_nft(ctx: Context<MintNft>) -> Result<()> {
        ctx.accounts.mint_nft()
    }

    pub fn freeze_nft(ctx: Context<FreezeNft>) -> Result<()> {
        ctx.accounts.freeze_nft()
    }

    pub fn thaw_nft(ctx: Context<ThawNft>) -> Result<()> {
        ctx.accounts.thaw_nft()
    }

    pub fn update_nft(ctx: Context<UpdateNft>, new_name: String) -> Result<()> {
        ctx.accounts.update_nft(new_name)
    }
}
