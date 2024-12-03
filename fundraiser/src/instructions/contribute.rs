use crate::constants::MIN_AMOUNT_TO_RAISE;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_token::instructions::Transfer;

pub fn process_contribute_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let amount: u64 = unsafe { *(data.as_ptr() as *const u64) };
    assert!(amount >= MIN_AMOUNT_TO_RAISE, "Amount too low");

    let [signer, contributor, signer_ta, fundraiser, vault, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Transfer {
        from: signer_ta,
        to: vault,
        authority: signer,
        amount,
    }
    .invoke()?;

    unsafe {
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(0) as *mut u64) += amount;
        *(contributor.borrow_mut_data_unchecked().as_mut_ptr() as *mut u64) += amount;
    }

    Ok(())
}
