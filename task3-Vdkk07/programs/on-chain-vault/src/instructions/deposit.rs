//-------------------------------------------------------------------------------
///
/// TASK: Implement the deposit functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the user has enough balance to deposit
/// - Verify that the vault is not locked
/// - Transfer lamports from user to vault using CPI (Cross-Program Invocation)
/// - Emit a deposit event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::DepositEvent;
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let authority = &ctx.accounts.user;
    let system_program = &ctx.accounts.system_program;

    if vault.locked {
        return Err(VaultError::VaultLocked.into());
    }

    let authority_lamports = **authority.lamports.borrow();
    if authority_lamports < amount {
        return Err(VaultError::InsufficientBalance.into());
    }

    let ixn = transfer(&authority.key(), &vault.key(), amount);

    invoke(
        &ixn,
        &[
            authority.to_account_info(),
            vault.to_account_info(),
            system_program.to_account_info(),
        ],
    )?;

    emit!(DepositEvent {
        vault: vault.key(),
        user: authority.key(),
        amount,
    });

    Ok(())
}
