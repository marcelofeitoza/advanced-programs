use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::instructions;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct EscrowArgs {
    pub maker: Pubkey,
    pub mint_b: Pubkey,
    pub amount: u64,
    pub receive: u64,
    pub escrow_bump: u8,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum EscrowInstruction {
    Make(EscrowArgs),
    Take(EscrowArgs),
    Refund(EscrowArgs),
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let instruction = EscrowInstruction::try_from_slice(data)?;

    match instruction {
        EscrowInstruction::Make(escrow_args) => {
            instructions::make(program_id, accounts, escrow_args)
        }
        EscrowInstruction::Take(escrow_args) => {
            instructions::take(program_id, accounts, escrow_args)
        }
        EscrowInstruction::Refund(escrow_args) => {
            instructions::refund(program_id, accounts, escrow_args)
        }
    }
}
