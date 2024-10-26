use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::processor::EscrowArgs;

pub fn take(program_id: &Pubkey, accounts: &[AccountInfo], args: EscrowArgs) -> ProgramResult {
    let [maker, mint_a, mint_b] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Ok(())
}
