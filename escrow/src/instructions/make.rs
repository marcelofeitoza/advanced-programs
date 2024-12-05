use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

pub fn process(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, escrow] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(maker.is_signer(), "Maker is not signer");
    unsafe {
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().add(0) as *mut Pubkey) = *(maker.key());
    }

    unsafe {
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().add(32) as *mut Pubkey) =
            *((data.as_ptr()).add(0) as *const Pubkey); // maker_ta_b
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().add(64) as *mut Pubkey) =
            *((data.as_ptr()).add(32) as *const Pubkey); // mint_a
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().add(96) as *mut Pubkey) =
            *((data.as_ptr()).add(64) as *const Pubkey); // mint_b
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().add(128) as *mut u64) =
            *((data.as_ptr()).add(96) as *const u64); // amount_b
    }

    Ok(())
}
