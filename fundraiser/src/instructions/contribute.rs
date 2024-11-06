use crate::constants::{MAX_CONTRIBUTION_PERCENTAGE, PERCENTAGE_SCALER, SECONDS_TO_DAYS};
use crate::errors::FundraiserError;
use crate::state::{Contributor, Fundraiser};
use pinocchio::sysvars::Sysvar;
use pinocchio::{
    account_info::AccountInfo, msg, program_error::ProgramError, sysvars::clock::Clock,
    ProgramResult,
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

    msg!("Amount from data: {}", amount);

    Transfer {
        from: contributor_ata,
        to: vault,
        authority: contributor,
        amount,
    }
    .invoke()?;

    // unsafe { fundraiser.borrow_mut_data_unchecked()[64..72].copy_from_slice(&(fundraiser_account.remaining_amount() - amount).to_le_bytes()); }
    let fundraiser_account_data = Fundraiser::from_account_info(fundraiser)?;
    let contributor_account_data = Contributor::from_account_info(contributor_account)?;
    unsafe {
        fundraiser.borrow_mut_data_unchecked()[72..80]
            .copy_from_slice(&(fundraiser_account_data.current_amount() + amount).to_le_bytes());
        msg!(
            "Fundraiser account current amount: {}",
            Fundraiser::from_account_info(fundraiser)?.current_amount()
        );

        contributor.borrow_mut_data_unchecked()[0..8]
            .copy_from_slice(&(contributor_account_data.amount() + amount).to_le_bytes());
        msg!(
            "Contributor account amount: {}",
            Contributor::from_account_info(contributor_account)?.amount()
        );
    }

    msg!("Contributed {} to the fundraiser", amount);

    Ok(())
}
