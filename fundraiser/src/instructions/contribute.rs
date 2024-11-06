use crate::constants::{MAX_CONTRIBUTION_PERCENTAGE, PERCENTAGE_SCALER, SECONDS_TO_DAYS};
use crate::errors::FundraiserError;
use pinocchio::sysvars::Sysvar;
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, sysvars::clock::Clock, ProgramResult,
};
use pinocchio_token::instructions::Transfer;

pub fn process_contribute_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [contributor, mint_to_raise, fundraiser, contributor_ata, contributor_account, vault] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if data.len() != 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let amount = u64::from_le_bytes(
        data.try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );
    let min_contribution = 1_u64;
    let max_contribution = unsafe {
        (*(fundraiser.borrow_data_unchecked().as_ptr().add(64) as *const u64)
            * MAX_CONTRIBUTION_PERCENTAGE)
            / PERCENTAGE_SCALER
    };
    let current_time = Clock::get()?.unix_timestamp;
    let fundraiser_end_time = unsafe {
        *(fundraiser.borrow_data_unchecked().as_ptr().add(80) as *const i64)
            + *(fundraiser.borrow_data_unchecked().as_ptr().add(88) as *const u8) as i64
                * SECONDS_TO_DAYS
    };

    if !contributor.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if amount < min_contribution {
        return Err(ProgramError::Custom(
            FundraiserError::ContributionTooSmall as u32,
        ));
    }
    if amount > max_contribution {
        return Err(ProgramError::Custom(
            FundraiserError::ContributionTooBig as u32,
        ));
    }
    if current_time > fundraiser_end_time {
        return Err(ProgramError::Custom(
            FundraiserError::FundraiserEnded as u32,
        ));
    }

    Transfer {
        from: contributor_ata,
        to: vault,
        authority: contributor,
        amount,
    }
    .invoke()?;

    unsafe {
        let current_amount_ptr =
            fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(72) as *mut u64;
        let current_amount = *current_amount_ptr;
        let new_amount = current_amount + amount;

        *current_amount_ptr = new_amount;
    }

    unsafe {
        contributor_account.borrow_mut_data_unchecked()[0..8]
            .copy_from_slice(&amount.to_le_bytes());
    }

    Ok(())
}
