use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer,transfer};

use crate::instructions::BuyEggs;
use crate::errors::SolanaFarmError;

pub fn transfer_to_vault<'info>(ctx: &Context<BuyEggs>, sol_value: u64) -> Result<()> {
    let solana_farm_vault = &ctx.accounts.solana_farm_vault;
    let signer = &ctx.accounts.user;
    let system_program = &ctx.accounts.system_program;

    let vault_balance_before = solana_farm_vault.get_lamports();

    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: signer.to_account_info(),
                to: solana_farm_vault.to_account_info(),
            },
        ),
        sol_value,
    )?;

    let vault_balance_after = solana_farm_vault.get_lamports();
    require_eq!(vault_balance_after, vault_balance_before + sol_value,SolanaFarmError::InvariantVaultBalance);

    Ok(())
}

pub fn transfer_from_vault<'info>(system_program: &Program<'info, System>,solana_farm_vault: &SystemAccount<'info>, recipient: &AccountInfo<'info>,return_lamports: u64, bumps: u8) -> Result<()> {
    let vault_balance_before = solana_farm_vault.get_lamports();

    let bump = &[bumps];
    let seeds: &[&[u8]] = &[b"solana_farm_vault".as_ref(), bump];
    let signer_seeds = &[&seeds[..]];

    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: solana_farm_vault.to_account_info(),
                to: recipient.to_account_info(),
            },
        ).with_signer(signer_seeds),
        return_lamports,
    )?;

    let vault_balance_after = solana_farm_vault.get_lamports();
    require_eq!(vault_balance_after, vault_balance_before - return_lamports,SolanaFarmError::InvariantVaultBalance);

    Ok(())
}