use crate::state::Fundraiser;
use pinocchio::{account_info::AccountInfo, msg, program_error::ProgramError, ProgramResult};
use pinocchio_token::instructions::Transfer;

pub fn process_contribute_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [contributor, _mint_to_raise, fundraiser, contributor_ata, contributor_account, vault, _token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let amount = u64::from_le_bytes(data[0..8].try_into().unwrap());
    msg!("Amount from data: {}", amount);

    // Transfer {
    //     from: contributor_ata,
    //     to: vault,
    //     authority: contributor,
    //     amount,
    // }
    // .invoke()
    // .map_err(|_| ProgramError::AccountBorrowFailed)?;

    let fundraiser_account_data = Fundraiser::from_account_info(fundraiser)?;
    unsafe {
        fundraiser.borrow_mut_data_unchecked()[72..80]
            .copy_from_slice(&(fundraiser_account_data.current_amount() + amount).to_le_bytes());
        msg!(
            "Fundraiser account current amount: {}",
            Fundraiser::from_account_info(fundraiser)?.current_amount()
        );

        *(contributor_account.borrow_mut_data_unchecked().as_mut_ptr() as *mut u64) =
            u64::from_le_bytes(amount.to_le_bytes());

        msg!("Contributed {} to the fundraiser", amount);
    }

    Ok(())
}
