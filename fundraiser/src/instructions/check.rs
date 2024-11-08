use crate::state::Fundraiser;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, signer, ProgramResult};
use pinocchio_token::instructions::Transfer;

pub fn process_check_instruction(accounts: &[AccountInfo]) -> ProgramResult {
    let [signer, signer_ta, fundraiser, vault, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let fundraiser_account = Fundraiser::from_account_info(fundraiser)?;
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
