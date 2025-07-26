# Pinocchio Vault

A minimalist Solana vault program built with Pinocchio's zero-copy framework that demonstrates native development without Anchor macros, using single-byte discriminators, manual account validation via TryFrom traits, and PDA-based security for deposit/withdraw operations.

## Overview

This project implements a simple lamport vault that allows users to securely store and withdraw SOL using Program Derived Addresses (PDAs). It serves as a practical example of building Solana programs with Pinocchio's lightweight, zero-copy approach.

## Key Features

- **Zero-copy optimization**: Direct byte slice access without memory allocation
- **Single-byte discriminators**: Efficient instruction routing (0 for Deposit, 1 for Withdraw)
- **Manual account validation**: Complete control over security checks via TryFrom traits
- **PDA-based security**: Program Derived Addresses ensure only authorized users can access their vaults
- **Cross-Program Invocation (CPI)**: Secure lamport transfers using System Program
- **No external dependencies**: Minimalist approach with zero dependency drag


## Instructions

### Deposit
- Validates vault is empty (prevents double deposits)
- Ensures deposit amount is greater than zero
- Transfers lamports from owner to vault using CPI
- Uses PDA derived from owner's public key

### Withdraw
- Verifies vault contains lamports
- Uses PDA signing to authorize the transfer
- Transfers all lamports back to the owner
- Ensures only the original depositor can withdraw

## Security Features

- **Signer validation**: Owner must sign all transactions
- **Account ownership checks**: Vault must be owned by System Program
- **PDA verification**: Vault address must match expected PDA derivation
- **Amount validation**: Prevents zero-amount transactions
- **State validation**: Prevents double deposits and empty withdrawals



## Learning Resources

This implementation is based on the [Pinocchio Vault Challenge](https://learn.blueshift.gg/en/challenges/pinocchio-vault) from Blueshift, which provides comprehensive guidance on building Solana programs with Pinocchio.

## Credits

- **Challenge Source**: [Blueshift Pinocchio Vault Challenge](https://learn.blueshift.gg/en/challenges/pinocchio-vault)

