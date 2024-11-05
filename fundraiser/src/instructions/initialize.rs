use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    ProgramResult,
};
use pinocchio::pubkey::Pubkey;
use crate::state::Fundraiser;

pub fn process_initialize_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, fundraiser] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    // With checks = 213 CUs | Without checks = 171 CUs
    assert_eq!(fundraiser.owner(), &crate::ID, "Invalid fundraiser account owner"); // 17 CUs
    assert_eq!(fundraiser.data_len(), Fundraiser::LEN, "Invalid fundraiser account data length"); // 3 CUs
    assert!(maker.is_signer(), "Maker account is not signer"); // 22 CUs

    unsafe {
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr() as *mut Pubkey) = *maker.key(); // Registers the maker pubkey on bytes 0-32 of the fundraiser account data
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(32) as *mut [u8; 49]) = *(data.as_ptr() as *const [u8; 49]); // Registers the rest of the data on bytes 32-81 of the fundraiser account data (mint_to_raise, amount_to_raise, time_started, duration)
    };

    Ok(())
}
