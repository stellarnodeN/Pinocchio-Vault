#![no_std]
#![allow(unexpected_cfgs)]
use pinocchio::{account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey, ProgramResult};
use pinocchio::{
 nostd_panic_handler
};
 
// Define the program entrypoint - this is where execution begins
entrypoint!(process_instruction);

// Use the no_std panic handler for Solana programs
nostd_panic_handler!();
 
// Import our instruction modules
pub mod instructions;
pub use instructions::*;
 

// Program ID - unique identifier for this deployed program
// This is a hardcoded public key that identifies our vault program
pub const ID: Pubkey = [
    0x0f, 0x1e, 0x6b, 0x14, 0x21, 0xc0, 0x4a, 0x07, 
    0x04, 0x31, 0x26, 0x5c, 0x19, 0xc5, 0xbb, 0xee, 
    0x19, 0x92, 0xba, 0xe8, 0xaf, 0xd1, 0xcd, 0x07, 
    0x8e, 0xf8, 0xaf, 0x70, 0x47, 0xdc, 0x11, 0xf7, 
];
 
 
// Main instruction processor - routes incoming instructions to appropriate handlers
// Uses single-byte discriminators (0 for Deposit, 1 for Withdraw) to identify instructions
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {    
    match instruction_data.split_first() {
        // Route to Deposit instruction handler (discriminator = 0)
        Some((Deposit::DISCRIMINATOR, data)) => Deposit::try_from((data, accounts))?.process(),
        // Route to Withdraw instruction handler (discriminator = 1)
        Some((Withdraw::DISCRIMINATOR, _)) => Withdraw::try_from(accounts)?.process(),
        // Invalid instruction if discriminator doesn't match known instructions
        _ => Err(ProgramError::InvalidInstructionData)
    }
}