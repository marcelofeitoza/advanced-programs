use pinocchio::account_info::AccountInfo;
use pinocchio::ProgramResult;

pub fn process_contribute_instruction(_accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    Ok(())
}
