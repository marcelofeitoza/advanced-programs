use crate::constants::{MAX_CONTRIBUTION_PERCENTAGE, PERCENTAGE_SCALER, SECONDS_TO_DAYS};
use crate::errors::FundraiserError;
use crate::state::Fundraiser;
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, sysvars::clock::Clock,
    sysvars::Sysvar, ProgramResult,
};
use pinocchio_token::instructions::Transfer;

pub fn process_contribute_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [contributor, mint_to_raise, fundraiser, contributor_account, contributor_ata, vault] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let amount = unsafe { *(data.as_ptr() as *const u64) };
    let fundraiser_data = Fundraiser::from_account_info(fundraiser)?;
    let min_contribution = 1_u64;
    let max_contribution =
        (fundraiser_data.amount_to_raise() * MAX_CONTRIBUTION_PERCENTAGE) / PERCENTAGE_SCALER;
    let current_time = unsafe { Clock::get()?.unix_timestamp };
    let fundraiser_end_time =
        fundraiser_data.time_started() + fundraiser_data.duration() as i64 * SECONDS_TO_DAYS;

    if !contributor.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if fundraiser_data.mint_to_raise() != *mint_to_raise.key() {
        return Err(ProgramError::InvalidAccountData);
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
        *(fundraiser.borrow_mut_data_unchecked().as_mut_ptr().add(72) as *mut u64) += amount;
        *(contributor_account.borrow_mut_data_unchecked().as_mut_ptr() as *mut u64) += amount;
    }

    Ok(())
}
