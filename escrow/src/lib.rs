use five8_const::decode_32_const;
use instructions::EscrowInstruction;
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

    match EscrowInstruction::try_from(instruction_discriminant)? {
        EscrowInstruction::Make => instructions::make::process(accounts, instruction_data),
        EscrowInstruction::Take => instructions::take::process(accounts, [instruction_data[0]]),
        EscrowInstruction::Refund => instructions::refund::process(accounts, [instruction_data[0]]),
    }
}
