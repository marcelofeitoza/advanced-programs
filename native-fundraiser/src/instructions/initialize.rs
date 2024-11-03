use crate::constants::MIN_AMOUNT_TO_RAISE;
use crate::state::Fundraiser;
use pinocchio::account_info::AccountInfo;
use pinocchio::instruction::{Seed, Signer};
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::{find_program_address, Pubkey};
use pinocchio::sysvars::clock::Clock;
use pinocchio::sysvars::rent::Rent;
use pinocchio::ProgramResult;

pub fn process_initialize_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, mint_to_raise, fundraiser, rent, clock] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let amount = unsafe { *(data.as_ptr() as *const u64) };
    let duration = unsafe { *(data.as_ptr().add(8)) };

    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (fundraiser_pda, fundraiser_bump) =
        find_program_address(&[b"fundraiser", maker.key().as_ref()], &crate::ID);

    if fundraiser.key().ne(&fundraiser_pda) {
        return Err(ProgramError::InvalidSeeds);
    }

    let clock = unsafe { (*(clock.borrow_data_unchecked().as_ptr() as *const Clock)).clone() };
    let rent = unsafe { (*(rent.borrow_data_unchecked().as_ptr() as *const Rent)).clone() };

    let time = clock.unix_timestamp;

    if amount <= MIN_AMOUNT_TO_RAISE {
        panic!(
            "Amount to raise must be greater than {}",
            MIN_AMOUNT_TO_RAISE
        );
    }

    let lamports = rent.minimum_balance(Fundraiser::LEN);

    let seed_fundraiser = Seed::from(b"fundraiser");
    let seed_maker = Seed::from(maker.key().as_ref());
    let fundraiser_bump_array = [fundraiser_bump];
    let seed_bump = Seed::from(&fundraiser_bump_array);

    let signer_seeds = [seed_fundraiser, seed_maker, seed_bump];
    let signer = Signer::from(&signer_seeds);

    pinocchio_system::instructions::CreateAccount {
        from: maker,
        to: fundraiser,
        lamports,
        space: Fundraiser::LEN as u64,
        owner: &crate::ID,
    }
    .invoke_signed(&[signer])?;

    unsafe {
        let fundraiser_data_ptr = fundraiser.borrow_mut_data_unchecked().as_mut_ptr();
        *(fundraiser_data_ptr.add(0) as *mut Pubkey) = *maker.key(); // maker
        *(fundraiser_data_ptr.add(32) as *mut Pubkey) = *mint_to_raise.key(); // mint_to_raise
        *(fundraiser_data_ptr.add(64) as *mut u64) = amount; // amount_to_raise
        *(fundraiser_data_ptr.add(72) as *mut u64) = 0; // current_amount
        *(fundraiser_data_ptr.add(80) as *mut i64) = time; // time_started
        *(fundraiser_data_ptr.add(88) as *mut u8) = duration; // duration
        *(fundraiser_data_ptr.add(89) as *mut u8) = fundraiser_bump; // bump
    }

    Ok(())
}
