use pinocchio::pubkey::Pubkey;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

pub fn process_initialize_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, fundraiser] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    assert!(maker.is_signer(), "Maker account is not signer");

    unsafe { *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr() as *mut Pubkey) = *maker.key() }
    unsafe { *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(32) as *mut [u8; 49]) = *(data.as_ptr() as *const [u8; 49])};

    Ok(())
}