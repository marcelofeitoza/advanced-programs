use crate::state::Fundraiser;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, signer, ProgramResult};
use pinocchio_token::instructions::Transfer;
use pinocchio_token::state::TokenAccount;

pub fn process_check_instruction(accounts: &[AccountInfo]) -> ProgramResult {
    let [signer, signer_ta, fundraiser, vault, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let fundraiser_account = Fundraiser::from_account_info(fundraiser)?;
    // let vault_ta = TokenAccount::from_account_info(vault)?;
    //
    // assert!(
    //     vault_ta.amount() >= fundraiser_account.amount_to_raise(),
    //     "Fundraiser not met"
    // );
    //
    let bump = fundraiser_account.bump();
    let fundraiser_seed = b"fundraiser".as_ref();
    let signer_key_seed = signer.key().as_ref();
    let bump_seed = &[bump];

    Transfer {
        from: vault,
        to: signer_ta,
        authority: fundraiser,
        amount: fundraiser_account.current_amount(),
    }
    .invoke_signed(&[signer!(fundraiser_seed, signer_key_seed, bump_seed)])?;

    Ok(())
}
