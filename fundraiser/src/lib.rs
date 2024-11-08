use crate::instructions::check::process_check_instruction;
use crate::instructions::contribute::process_contribute_instruction;
use crate::instructions::{process_initialize_instruction, FundraiserInstruction};
use five8_const::decode_32_const;
use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;
use pinocchio::{entrypoint, ProgramResult};

mod constants;
mod errors;
mod instructions;
mod state;

#[cfg(test)]
mod tests;

const ID: [u8; 32] = decode_32_const("22222222222222222222222222222222222222222222");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id != &ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (instruction_discrimnant, instruction_data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match FundraiserInstruction::try_from(instruction_discrimnant)? {
        FundraiserInstruction::InitializeInstruction => {
            process_initialize_instruction(accounts, instruction_data)?
        }
        FundraiserInstruction::ContributeInstruction => {
            process_contribute_instruction(accounts, instruction_data)?
        }
        FundraiserInstruction::CheckContributionsInstruction => {
            process_check_instruction(accounts)?
        }
        FundraiserInstruction::RefundInstruction => return Ok(()),
    }

    Ok(())
}
