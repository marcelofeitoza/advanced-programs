use crate::state::Fundraiser;
use pinocchio::pubkey::Pubkey;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

pub fn process_initialize_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, fundraiser] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert_eq!(
        fundraiser.owner(),
        &crate::ID,
        "Invalid fundraiser account owner"
    ); // 17 CUs
    assert_eq!(
        fundraiser.data_len(),
        Fundraiser::LEN,
        "Invalid fundraiser account data length"
    ); // 3 CUs
    assert!(maker.is_signer(), "Maker account is not signer"); // 22 CUs

    if data.len() != 49 {
        return Err(ProgramError::InvalidInstructionData);
    }

    unsafe {
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr() as *mut Pubkey) = *maker.key();
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(32) as *mut Pubkey) =
            data[0..32].try_into().unwrap();
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(64) as *mut u64) =
            u64::from_le_bytes(data[32..40].try_into().unwrap());
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(72) as *mut u64) = 0;
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(80) as *mut i64) =
            i64::from_le_bytes(data[40..48].try_into().unwrap());
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(88)) = data[48];
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(89)) = 0;
    }

    Ok(())
}
