use crate::constants::MIN_AMOUNT_TO_RAISE;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_token::instructions::Transfer;

pub fn process_contribute_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let amount: u64 = unsafe { *(data.as_ptr() as *const u64) };
    assert!(amount >= MIN_AMOUNT_TO_RAISE, "Amount too low"); // 2 CUs

    let [signer, contributor, signer_ta, fundraiser, vault, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // let fundraiser_account = Fundraiser::from_account_info_unchecked(
    //     fundraiser);
    // let contributor_account = Contributor::from_account_info_unchecked(contributor);
    // assert_eq!(
    //     &fundraiser_account.mint_to_raise(),
    //     mint.key(),
    //     "Invalid mint"
    // ); // 28 CUs
    // assert!(
    //     fundraiser_account.time_started() > 0,
    //     "Fundraiser not started yet"
    // ); // 2 CUs
    // assert!(
    //     fundraiser_account.time_started() + i64::from(fundraiser_account.duration()) > 0,
    //     "Fundraiser ended"
    // ); // 4 CUs

    Transfer {
        from: signer_ta,
        to: vault,
        authority: signer,
        amount,
    }
    .invoke()?; // 5K+ CUs

    unsafe {
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(72) as *mut u64) += amount;
        *(contributor.borrow_mut_data_unchecked().as_mut_ptr() as *mut u64) += amount;
    }

    Ok(())
}
