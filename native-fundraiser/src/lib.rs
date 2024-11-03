use five8_const::decode_32_const;
use pinocchio::account_info::AccountInfo;
use pinocchio::{entrypoint, ProgramResult};
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;
use crate::instructions::{process_check_contributions_instruction, process_contribute_instruction, process_initialize_instruction, process_refund_instruction, FundraiserInstruction};

mod constants;
mod errors;
mod instructions;
mod state;

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

    let (instruction_discrimnant, restinstruction_inner_data) = data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

    match FundraiserInstruction::try_from(instruction_discrimnant)? {
        FundraiserInstruction::InitializeInstruction => process_initialize_instruction(accounts, restinstruction_inner_data)?,
        FundraiserInstruction::ContributeInstruction => process_contribute_instruction(accounts, restinstruction_inner_data)?,
        FundraiserInstruction::CheckContributionsInstruction => process_check_contributions_instruction(accounts, restinstruction_inner_data)?,
        FundraiserInstruction::RefundInstruction => process_refund_instruction(accounts, restinstruction_inner_data)?,
    }

    Ok(())
}
