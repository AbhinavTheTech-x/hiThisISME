use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer,transfer};

use crate::instructions::BuyEggs;
use crate::errors::MangoFarmError;

pub fn transfer_to_vault<'info>(ctx: &Context<BuyEggs>, sol_value: u64) -> Result<()> {
    let mango_farm_vault = &ctx.accounts.mango_farm_vault;
    let signer = &ctx.accounts.user;
    let system_program = &ctx.accounts.system_program;

    let vault_balance_before = mango_farm_vault.get_lamports();

    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: signer.to_account_info(),
                to: mango_farm_vault.to_account_info(),
            },
        ),
        sol_value,
    )?;

    let vault_balance_after = mango_farm_vault.get_lamports();
    require_eq!(vault_balance_after, vault_balance_before + sol_value,MangoFarmError::InvariantVaultBalance);

    Ok(())
}

pub fn transfer_from_vault<'info>(system_program: &Program<'info, System>,mango_farm_vault: &SystemAccount<'info>, recipient: &AccountInfo<'info>,return_lamports: u64, bumps: u8) -> Result<()> {
    let vault_balance_before = mango_farm_vault.get_lamports();

    let bump = &[bumps];
    let seeds: &[&[u8]] = &[b"mango_farm_vault".as_ref(), bump];
    let signer_seeds = &[&seeds[..]];

    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: mango_farm_vault.to_account_info(),
                to: recipient.to_account_info(),
            },
        ).with_signer(signer_seeds),
        return_lamports,
    )?;

    let vault_balance_after = mango_farm_vault.get_lamports();
    require_eq!(vault_balance_after, vault_balance_before - return_lamports,MangoFarmError::InvariantVaultBalance);

    Ok(())
}