use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, sysvars::clock::Clock,
    ProgramResult,
};

pub fn process_initialize_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [fundraiser, clock] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert_eq!(
        *clock.key(),
        Pubkey::from(five8_const::decode_32_const(
            "SysvarC1ock11111111111111111111111111111111"
        )),
        "Clock account is not sysvar clock"
    );

    unsafe {
        let clock = &*(clock.borrow_data_unchecked().as_ptr() as *const Clock);
        let fundraiser_account = fundraiser.borrow_mut_data_unchecked().as_mut_ptr();
        *(fundraiser_account.add(8) as *mut i64) = clock.unix_timestamp; // time started
        *(fundraiser_account.add(16) as *mut Pubkey) = *((data.as_ptr()).add(0) as *const Pubkey); // maker
        *(fundraiser_account.add(48) as *mut Pubkey) = *((data.as_ptr()).add(32) as *const Pubkey); // mint
        *(fundraiser_account.add(80) as *mut u64) = *((data.as_ptr()).add(64) as *const u64); // amount to raise
        *(fundraiser_account.add(88) as *mut u64) = *((data.as_ptr()).add(72) as *const u64); // duration
        *(fundraiser_account.add(89)) = *((data.as_ptr()).add(73)); // bump
    }

    Ok(())
}
