use anchor_lang::prelude::*;

use mpl_core::{
    ID as MPL_CORE_PROGRAM_ID,
    accounts::BaseCollectionV1, 
    types::{PluginAuthorityPair, Plugin, PermanentFreezeDelegate, Attributes, Attribute}, 
    instructions::{CreateV2CpiBuilder, CreateCollectionV2CpiBuilder}, 
};
declare_id!("9jUWKwf6iVuDoFYFJhY84NXUot3Vrqz4Jh39YSn7YK86");

#[program]
pub mod anchor_example {
    use mpl_core::{types::PluginAuthority, AuthorityType};

    use super::*;

    pub fn create_collection(ctx: Context<CreateCollection>) -> Result<()> {

        let mut collection_plugins = vec![];

        collection_plugins.push( PluginAuthorityPair { plugin: Plugin::PermanentFreezeDelegate( PermanentFreezeDelegate { frozen: true}), authority: None});
        // collection_plugins.push(PluginAuthorityPair);

        CreateCollectionV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .collection(&ctx.accounts.collection.to_account_info())
        .payer(&ctx.accounts.payer.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .update_authority(Some(&ctx.accounts.reputation_config.to_account_info()))
        .name("Solaura official reputation NFT collection".to_string())
        .uri("https://example.com".to_string())
        .plugins(collection_plugins)
        .invoke()?;

        // ctx.accounts.reputation_config.bump = ctx.bumps.reputation_config;   it should be implemented for the new collection

        Ok(())
    }

    pub fn create_asset(ctx: Context<CreateAsset>) -> Result<()> {

       ctx.accounts.reputation_config.set_inner(ReputationConfig{
        user: ctx.accounts.payer.key(),
        bump: ctx.accounts.reputation_config.bump,
        reputation_nft_mint: ctx.accounts.asset.key(),
        reputation: 100,
       });
 

        let seeds = &[
            &"reputation_config".as_bytes(),
            &ctx.accounts.payer.key().to_bytes()[..],
            &[ctx.accounts.reputation_config.bump]];

        let signer_seeds = &[&seeds[..]];

        let mut collection_plugin = vec![];

        // collection_plugins.push( 
        //     PluginAuthorityPair { 
        //         plugin: Plugin::PermanentFreezeDelegate( 
        //             PermanentFreezeDelegate { 
        //                 frozen: true
        //             }
        //         ), 
        //         authority: Some(PluginAuthority::Address { 
        //             address: ctx.accounts.reputation_config.key()}
        //         )
        //     }
        // );

        // collection_plugins.push(AttributesPlugin {
        //     });

        let attribute_list: Vec<Attribute> = vec![
            Attribute {
                key: "Reputation".to_string(),
                value: 1000.to_string() 
            },
        ];

        collection_plugin.push(PluginAuthorityPair {
            plugin: Plugin::Attributes(Attributes { attribute_list}),
            authority: Some(PluginAuthority::UpdateAuthority)
        });

        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .asset(&ctx.accounts.asset.to_account_info())
        // .collection(Some(&ctx.accounts.collection.to_account_info()))
        .payer(&ctx.accounts.payer.to_account_info())
        .update_authority(Some(&ctx.accounts.reputation_config.to_account_info()))
        .system_program(&ctx.accounts.system_program.to_account_info())
        .name("My Super Asset".to_string())
        .uri("https://arweave.net/32YZkZ4iXbhqHPcPkHHX8xpvNywp9hwt2SNDrbCmJZyg".to_string())
        .plugins(collection_plugin)
        .invoke_signed(signer_seeds)?;
    // .authority(Some(&ctx.accounts.reputation_config.to_account_info()))


    

        Ok(())
    }

}

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub collection: Signer<'info>,
    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: This doesn't need to be checked, because there is the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        // init,
        // payer = payer,
        // space = ReputationConfig::INIT_SPACE,
        seeds = [
            b"reputation_config".as_ref(),
            payer.key().as_ref(),
        ],
        bump = reputation_config.bump   
    )]
    pub reputation_config: Account<'info, ReputationConfig>,
}

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        constraint = collection.update_authority == reputation_config.key(),
    )]
    pub collection: Account<'info, BaseCollectionV1>,
    #[account(mut)]
    pub asset: Signer<'info>,
    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: This doesn't need to be checked, because there is the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        seeds = [
            b"reputation_config".as_ref(),
            payer.key().as_ref(),
        ],
        bump = reputation_config.bump
    )]
    pub reputation_config: Account<'info, ReputationConfig>,
}

#[account]
pub struct ReputationConfig {
    pub user: Pubkey,
    pub bump: u8,
    pub reputation_nft_mint: Pubkey,
    pub reputation: u64,
}

impl Space for ReputationConfig {
    const INIT_SPACE: usize = 8 + 32 + 1 + 32 + 8;
}

