use crate::instructions::{
    check::process_check_instruction, contribute::process_contribute_instruction,
    initialize::process_initialize_instruction, refund::process_refund_instruction,
    FundraiserInstruction,
};
use five8_const::decode_32_const;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    {entrypoint, ProgramResult},
};

mod constants;
mod errors;
mod instructions;
mod state;
#[cfg(test)]
mod tests;

const ID: [u8; 32] = decode_32_const("99999999999999999999999999999999999999999999");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id != &ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (instruction_discriminant, instruction_data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match FundraiserInstruction::try_from(instruction_discriminant)? {
        FundraiserInstruction::Initialize => {
            process_initialize_instruction(accounts, instruction_data)?
        }
        FundraiserInstruction::Contribute => {
            process_contribute_instruction(accounts, instruction_data)?
        }
        FundraiserInstruction::CheckContributions => process_check_instruction(accounts)?,
        FundraiserInstruction::Refund => process_refund_instruction(accounts)?,
    }

    Ok(())
}
