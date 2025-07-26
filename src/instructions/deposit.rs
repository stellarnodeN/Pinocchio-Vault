// Deposit instruction: handles user deposits into the vault
// Validates accounts and instruction data, then performs the lamport transfer

use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::find_program_address, ProgramResult};
use pinocchio_system::instructions::Transfer;
use core::mem::size_of;

// Account struct for deposit instruction - contains owner and vault accounts
pub struct DepositAccounts<'a> {
    pub owner: &'a AccountInfo,  // User making the deposit (must be signer)
    pub vault: &'a AccountInfo,  // PDA vault account to receive lamports
}
 
impl<'a> TryFrom<&'a [AccountInfo]> for DepositAccounts<'a> {
    type Error = ProgramError;
 
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        // Destructure accounts slice - expect owner, vault, and system program
        let [owner, vault, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
 
        // Account validation checks
        if !owner.is_signer() {
            return Err(ProgramError::InvalidAccountOwner);  // Owner must sign the transaction
        }
 
        if unsafe { vault.owner().ne(&pinocchio_system::ID) } {
            return Err(ProgramError::InvalidAccountOwner);  // Vault must be owned by System Program
        }
 
        if vault.lamports().ne(&0) {
            return Err(ProgramError::InvalidAccountData);  // Vault must be empty (no double deposits)
        }
 
        // Verify vault is the correct PDA for this owner
        let (vault_key, _) = find_program_address(&[b"vault", owner.key()], &crate::ID);
        if vault.key().ne(&vault_key) {
            return Err(ProgramError::InvalidAccountOwner);
        }
 
        // Return validated accounts
        Ok(Self { owner, vault })
    }
}

// Instruction data struct - contains the deposit amount
pub struct DepositInstructionData {
    pub amount: u64,  // Amount of lamports to deposit
}
 
impl<'a> TryFrom<&'a [u8]> for DepositInstructionData {
    type Error = ProgramError;
 
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        // Verify data length matches u64 size (8 bytes)
        if data.len() != size_of::<u64>() {
            return Err(ProgramError::InvalidInstructionData);
        }
 
        // Convert byte slice to u64 (little-endian)
        let amount = u64::from_le_bytes(data.try_into().unwrap());
 
        // Instruction validation
        if amount.eq(&0) {
            return Err(ProgramError::InvalidInstructionData);  // Amount must be greater than 0
        }
 
        Ok(Self { amount })
    }
}

// Main deposit instruction struct - combines accounts and instruction data
pub struct Deposit<'a> {
    pub accounts: DepositAccounts<'a>,
    pub instruction_datas: DepositInstructionData,
}
 
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Deposit<'a> {
    type Error = ProgramError;
 
    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        // Validate accounts and instruction data
        let accounts = DepositAccounts::try_from(accounts)?;
        let instruction_datas: DepositInstructionData = DepositInstructionData::try_from(data)?;
 
        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}
 
impl<'a> Deposit<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;  // Single-byte discriminator for deposit instruction
 
    pub fn process(&mut self) -> ProgramResult {
        // Perform the lamport transfer from owner to vault using CPI
        Transfer {
            from: self.accounts.owner,  // Source account (owner)
            to: self.accounts.vault,    // Destination account (vault)
            lamports: self.instruction_datas.amount,  // Amount to transfer
        }
        .invoke()?;  // Execute the transfer
 
        Ok(())
    }
}



