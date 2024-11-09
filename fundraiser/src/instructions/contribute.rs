use crate::constants::MIN_AMOUNT_TO_RAISE;
use crate::state::Fundraiser;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio::sysvars::clock::Clock;
use pinocchio::sysvars::Sysvar;
use pinocchio_token::instructions::Transfer;

pub fn process_contribute_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let amount: u64 = u64::from_le_bytes(data[0..8].try_into().unwrap());
    assert!(amount >= MIN_AMOUNT_TO_RAISE, "Amount too low");

    let [signer, contributor, signer_ta, fundraiser, vault, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let mut fundraiser_account = Fundraiser::from_account_info_unchecked(fundraiser);

    let current_slot = Clock::get().expect("Failed to load the clock").slot;
    assert!(current_slot <= fundraiser_account.end_time() as u64, "Fundraiser ended");

    Transfer {
        from: signer_ta,
        to: vault,
        authority: signer,
        amount,
    }
        .invoke()?;

    unsafe {
        let remaining_amount = fundraiser_account.remaining_amount() - amount;
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(64) as *mut [u8; 8]) = remaining_amount.to_le_bytes();
        *(contributor.borrow_mut_data_unchecked().as_mut_ptr() as *mut u64) += amount;
        fundraiser_account.set_remaining_amount(remaining_amount);
    }

    Ok(())
}