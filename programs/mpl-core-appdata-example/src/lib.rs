use anchor_lang::prelude::*;

use mpl_core::{ instructions::CreateV2CpiBuilder, ID as MPL_CORE_ID };

declare_id!("BicWwtfJJAzWfAp2hzpdnyvvjB5TKnikAXxLVZbHcM2U");

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateAssetArgs {
    pub name: String,
    pub uri: String,
}

#[program]
pub mod mpl_core_appdata_example {
    use super::*;

    pub fn setup_manager(ctx: Context<SetupManager>) -> Result<()> {
        ctx.accounts.manager.bump = ctx.bumps.manager;

        Ok(())
    }

    pub fn create_asset(ctx: Context<CreateAsset>, args: CreateAssetArgs) -> Result<()> {
        let signer_seeds = &[b"manager".as_ref(), &[ctx.accounts.manager.bump]];

        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .payer(&ctx.accounts.payer.to_account_info())
            .authority(Some(&ctx.accounts.manager.to_account_info()))
            .owner(Some(&ctx.accounts.signer.to_account_info()))
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .invoke_signed(&[signer_seeds])?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupManager<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = Manager::INIT_SPACE,
        seeds = [MANAGER_SEEDS.as_bytes()],
        bump
    )]
    pub manager: Account<'info, Manager>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds = [MANAGER_SEEDS.as_bytes()], bump = manager.bump)]
    pub manager: Account<'info, Manager>,
    #[account(mut)]
    pub asset: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_ID)]
    /// CHECK: This is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}

const MANAGER_SEEDS: &str = "manager";

#[account]
pub struct Manager {
    pub bump: u8,
}

impl Space for Manager {
    const INIT_SPACE: usize = 8 + 1;
}
