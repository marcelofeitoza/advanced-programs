use pinocchio::pubkey::Pubkey;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

pub fn process_initialize_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [fundraiser] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    unsafe {
        let fundraiser_account = fundraiser.borrow_mut_data_unchecked().as_mut_ptr();

        // Cost: 113 CUs
        *(fundraiser_account.add(8) as *mut u64) = *((data.as_ptr()) as *const u64); // time started
        *(fundraiser_account.add(16) as *mut Pubkey) = *((data.as_ptr()).add(8) as *const Pubkey); // maker
        *(fundraiser_account.add(48) as *mut Pubkey) = *((data.as_ptr()).add(40) as *const Pubkey); // mint
        *(fundraiser_account.add(80) as *mut u64) = *((data.as_ptr()).add(72) as *const u64); // amount to raise
        *(fundraiser_account.add(88) as *mut u64) = *((data.as_ptr()).add(80) as *const u64); // duration
        *(fundraiser_account.add(89)) = *((data.as_ptr()).add(81)); // bump

        // Cost: 212 CUs
        // *(fundraiser_account.add(8) as *mut [u8; 82]) = *(data.as_ptr() as *const [u8; 82]);
    }

    Ok(())
}
