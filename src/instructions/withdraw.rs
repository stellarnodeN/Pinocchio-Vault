// Withdraw instruction: handles user withdrawals from the vault
// Validates accounts, ensures only the owner can withdraw, and performs the lamport transfer

use pinocchio::{account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::find_program_address, ProgramResult};
use pinocchio_system::instructions::Transfer;

// Account struct for withdraw instruction - contains owner, vault, and bump seed
pub struct WithdrawAccounts<'a> {
    pub owner: &'a AccountInfo,  // User withdrawing funds (must be signer)
    pub vault: &'a AccountInfo,  // PDA vault account containing the lamports
    pub bumps: [u8; 1],          // Bump seed for PDA signing
}
 
// Perform sanity checks on the accounts
impl<'a> TryFrom<&'a [AccountInfo]> for WithdrawAccounts<'a> {
    type Error = ProgramError;
 
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        // Destructure accounts slice - expect owner, vault, and system program
        let [owner, vault, _system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
 
        // Basic account validation checks
        if !owner.is_signer() {
            return Err(ProgramError::InvalidAccountOwner);  // Owner must sign the transaction
        }
 
        if unsafe { vault.owner() } != &pinocchio_system::ID {
            return Err(ProgramError::InvalidAccountOwner);  // Vault must be owned by System Program
        }
 
        // Verify vault is the correct PDA for this owner and get bump seed
        let (vault_key, bump) = find_program_address(&[b"vault", owner.key().as_ref()], &crate::ID);
        if &vault_key != vault.key() {
            return Err(ProgramError::InvalidAccountOwner);  // Vault must match expected PDA
        } 
 
        Ok(Self { owner, vault, bumps: [bump] })
    }
}

// Main withdraw instruction struct - contains validated accounts
pub struct Withdraw<'a> {
    pub accounts: WithdrawAccounts<'a>,
}
 
impl<'a> TryFrom<&'a [AccountInfo]> for Withdraw<'a> {
    type Error = ProgramError;
 
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        // Validate accounts
        let accounts = WithdrawAccounts::try_from(accounts)?;
 
        Ok(Self { accounts })
    }
}
 
impl<'a> Withdraw<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;  // Single-byte discriminator for withdraw instruction
 
    pub fn process(&mut self) -> ProgramResult {
        // Create signer seeds for PDA signing - allows vault to sign the transfer
        let seeds = [
            Seed::from(b"vault"),                                    // Seed: "vault"
            Seed::from(self.accounts.owner.key().as_ref()),          // Seed: owner's public key
            Seed::from(&self.accounts.bumps),                        // Seed: bump
        ];
        let signers = [Signer::from(&seeds)];  // Create signer from seeds
 
        // Transfer all lamports from vault back to owner using signed CPI
        Transfer {
            from: self.accounts.vault,                    // Source account (vault)
            to: self.accounts.owner,                      // Destination account (owner)
            lamports: self.accounts.vault.lamports(),     // Transfer all available lamports
        }
        .invoke_signed(&signers)?;  // Execute transfer with PDA signature
 
        Ok(())
    }
}