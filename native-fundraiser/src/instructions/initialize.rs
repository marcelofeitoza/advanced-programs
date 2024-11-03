use pinocchio::account_info::AccountInfo;
use pinocchio::ProgramResult;
use pinocchio::instruction::{Seed, Signer};
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;
use pinocchio::sysvars::clock::Clock;
use pinocchio::sysvars::rent::Rent;
use crate::state::{Contributor, Fundraiser};

pub fn process_initialize_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        maker,
        fundraiser,
        vault,
        contributor,
        rent,
        clock
    ] = accounts else { return Err(ProgramError::NotEnoughAccountKeys) };
    let (fundraiser_pda, fundraiser_bump_seed) = Pubkey::find_program_address(
        &[b"fundraiser", maker.key().as_ref()],
        &crate::ID,
    );
    let (contributor_pda, contributor_bump_seed) = Pubkey::find_program_address(
        &[b"contributor", fundraiser.key().as_ref(), maker.key().as_ref()],
        &crate::ID,
    );

    let mut fundraiser_data = Fundraiser::from_account_info(fundraiser)?;
    let mut contributor_data = Contributor::from_account_info(contributor)?;

    let amount = unsafe { *(data.get_unchecked(0) as *const u8 as *const u64) };
    let duration = unsafe { *(data.get_unchecked(8) as *const u8) };
    let fundraiser_bump = unsafe { *(data.get_unchecked(9) as *const u8) };

    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if fundraiser.key().ne(&fundraiser_pda) {
        return Err(ProgramError::InvalidAccountData);
    }

    const SYSVAR_RENT_PUBKEY: Pubkey = Pubkey::new_from_array(five8_const::decode_32_const(
        "SysvarRent111111111111111111111111111111111",
    ));
    const SYSVAR_CLOCK_PUBKEY: Pubkey = Pubkey::new_from_array(five8_const::decode_32_const(
        "SysvarC1ock11111111111111111111111111111111",
    ));

    if *rent.key().ne(&SYSVAR_RENT_PUBKEY) {
        return Err(ProgramError::InvalidAccountData);
    }
    if *clock.key().ne(&SYSVAR_CLOCK_PUBKEY) {
        return Err(ProgramError::InvalidAccountData);
    }

    let rent = unsafe { (*(rent.borrow_data_unchecked().as_ptr() as *const Rent)).clone() };
    let clock = unsafe { (*(clock.borrow_data_unchecked().as_ptr() as *const Clock)).clone() };
    let lamports = rent.minimum_balance(Fundraiser::LEN);
    let time = clock.unix_timestamp;

    unsafe {
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr() as *mut Pubkey) = *maker.key();
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(32) as *mut i64) = time;
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(40) as *mut [u8; 42]) =
            *(data.as_ptr() as *const [u8; 42]);
    }


    let signer_seed = [Seed::from(&[b"fundraiser", maker.key().as_ref(), &[fundraiser_bump_seed]])];
    let signer = [Signer::from(&signer_seed)];
    pinocchio_system::instructions::CreateAccount {
        from: maker,
        to: fundraiser,
        lamports,
        space: Fundraiser::LEN as u64,
        owner: &crate::ID,
    }
        .invoke_signed(&signer)?;

    // let fundraiser_data = fundraiser.try_borrow_mut_data()?;
    // unsafe {
    //     let dst = fundraiser_data.as_mut_ptr();
    //     // Copy maker Pubkey
    //     ptr::copy_nonoverlapping(maker.key().as_ref().as_ptr(), dst, 32);
    //     // Set time_started (i64)
    //     ptr::write_unaligned(dst.add(32) as *mut i64, time);
    //     // Copy amount_to_raise (u64)
    //     ptr::write_unaligned(dst.add(40) as *mut u64, amount);
    //     // Set current_amount to zero (u64)
    //     ptr::write_unaligned(dst.add(48) as *mut u64, 0u64);
    //     // Set duration (u8)
    //     ptr::write_unaligned(dst.add(56) as *mut u8, duration);
    //     // Set bump (u8)
    //     ptr::write_unaligned(dst.add(57) as *mut u8, fundraiser_bump_seed);
    // }
    // unsafe {
    //     *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(72) as *mut u64) =
    //         (*(fundraiser.borrow_data_unchecked().as_ptr().add(72) as *const u64))
    //             .checked_add(amount)
    //             .ok_or(ProgramError::ArithmeticOverflow)?;
    // }
    let fundraiser_data = Fundraiser::from_account_info(fundraiser)?;
    if fundraiser_data.amount_to_raise().checked_add(amount).is_none() {
        return Err(ProgramError::ArithmeticOverflow);
    }
    unsafe {
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(72) as *mut u64) =
            (*(fundraiser.borrow_data_unchecked().as_ptr().add(72) as *const u64))
                .checked_add(amount)
                .ok_or(ProgramError::ArithmeticOverflow)?;
    }
    unsafe {
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(56) as *mut u8) = duration;
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(57) as *mut u8) = fundraiser_bump;
    }

    Ok(())
}

