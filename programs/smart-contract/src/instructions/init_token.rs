use crate::utils::ADMIN_PUBKEY;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use mpl_token_metadata::instructions::CreateV1CpiBuilder;

pub fn init_token(ctx: Context<InitToken>) -> Result<()> {
    let seed = b"token";
    let bump = ctx.bumps.token_mint;
    let signer: &[&[&[u8]]] = &[&[seed, &[bump]]];

    CreateV1CpiBuilder::new(ctx.accounts.token_program.to_account_info().as_ref())
        .metadata(ctx.accounts.metadata.to_account_info().as_ref())
        .authority(ctx.accounts.token_mint.to_account_info().as_ref())
        .mint(ctx.accounts.token_mint.to_account_info().as_ref(), false)
        .update_authority(ctx.accounts.token_mint.to_account_info().as_ref(), true)
        .system_program(ctx.accounts.system_program.to_account_info().as_ref())
        .payer(ctx.accounts.admin.to_account_info().as_ref())
        .name(String::from("Token"))
        .uri(String::from("uri"))
        .is_mutable(true)
        .decimals(9)
        .symbol(String::from("tkn"))
        .token_standard(mpl_token_metadata::types::TokenStandard::Fungible)
        .invoke_signed(signer)?;

    Ok(())
}

#[derive(Accounts)]
pub struct InitToken<'info> {
    /// CHECK: New Metaplex Account being created
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    #[account(mut, address = ADMIN_PUBKEY)]
    pub admin: Signer<'info>,
    #[account(
      init,
      seeds=[b"token"],
      bump,
      payer=admin,
      mint::decimals=9,
      mint::authority=token_mint
    )]
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
